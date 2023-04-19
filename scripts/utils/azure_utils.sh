#!/bin/bash

# az ssh vm --name sgx2 --local-user mgild

# Utils to help create and manage an Azure SGX virtual machine
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils.sh"

function azure_verify_cli_installed() {
    install_dependency "azure-cli" "az"
}

# Get a Virtual Machine's IP from its name
function azure_get_vm_ip() {
    local vm_name=$1
    if [ -z "$vm_name" ]; then
        echo -e "${Red}VM_NAME was not provided${Color_Off}"
        return 1
    fi

    local vm_ip;
    vm_ip=$(az vm list-ip-addresses --name "$AZURE_VM_NAME" | jq '.[0].virtualMachine.network.publicIpAddresses[0].ipAddress' | tr -d '"')
    if [ -z "$vm_ip" ]; then
        echo -e "${Red}Failed to find IP address for ${Blue}$vm_name${Color_Off}"
        return 1
    fi

    echo "$vm_ip"
}

# Check if a virtual machine with a given name exists
function azure_check_vm_exists() {
    local vm_name=$1
    if [ -z "$vm_name" ]; then
        echo -e "${Red}VM_NAME was not provided${Color_Off}"
        return 1
    fi

    vm_list=$(az vm list)
    vm_count=$(echo "$vm_list" | jq --arg AZURE_VM_NAME "$AZURE_VM_NAME" '.[] | select(.name == $AZURE_VM_NAME) | .name' | wc -l)
    [ "$vm_count" -gt 0 ] && echo "true" || echo "false"
}

# Install Ubuntu dependencies in an Azure virtual machine
function azure_setup_vm_deps() {
    local vm_ip=$1
    if [ -z "$vm_ip" ]; then
        echo -e "${Red}VM_IP was not provided${Color_Off}"
        return 1
    fi

    # Copy and install the dependency build script
    scp -r -q -o LogLevel=QUIET "$(normalize_project_path scripts/vm)"/* "$AZURE_VM_ADMIN_USER"@"$vm_ip:/home/$AZURE_VM_ADMIN_USER"
    ssh "$AZURE_VM_ADMIN_USER"@"$vm_ip" "bash /home/$AZURE_VM_ADMIN_USER/setup_deps.sh $AZURE_VM_ADMIN_USER"
}

# Create a new Azure virtual machine with a given name
function azure_create_vm() {
    local vm_name=$1
    if [ -z "$vm_name" ]; then
        echo -e "${Red}VM_NAME was not provided${Color_Off}"
        return 1
    fi
    local ssh_key_name="${2:-id_azure_rsa}"
    local ssh_key_path="$HOME/.ssh/$ssh_key_name.pub"

    create_vm_response=$(az vm create \
        -n "$vm_name" \
        -g Default \
        --admin-username "$AZURE_VM_ADMIN_USER" \
        --size "$AZURE_VM_SIZE" \
        --generate-ssh-keys \
        --location "$AZURE_VM_LOCATION" \
        --zone 2 \
        --ssh-key-values "$ssh_key_path" \
        --image Canonical:0001-com-ubuntu-minimal-focal:minimal-20_04-lts-gen2:20.04.202303290)
    
    # Attempt to get the VM IP and add to our SSH known hosts
    vm_public_ip=$(echo "$create_vm_response" | jq -r '.publicIpAddress')
    if [ -n "$vm_public_ip" ]; then
        # setup SSH config files
        ssh-keyscan -H "$vm_public_ip" >> ~/.ssh/known_hosts
        ssh_add_host_config "$vm_name" "$vm_public_ip" "$AZURE_VM_ADMIN_USER" "$AZURE_SSH_KEY"
        azure_setup_vm_deps "$vm_public_ip"
    fi
}

# Setup the SGX measurement within an Azure virtual machine
function azure_setup_vm_measurement() {
    local vm_ip=$1
    if [ -z "$vm_ip" ]; then
        echo -e "${Red}VM_IP was not provided${Color_Off}"
        return 1
    fi

    local img_name=$2
    if [ -z "$img_name" ]; then
        echo -e "${Red}SGX_IMG_NAME was not provided${Color_Off}"
        return 1
    fi

   ssh "$AZURE_VM_ADMIN_USER"@"$vm_ip" << EOF
        printf '\n\nStarting docker service ...\n'
        sudo systemctl start docker
        sudo systemctl status docker
        docker ps
        if [ ! -d sbv3-example1 ]; then
            git clone https://github.com/switchboard-xyz/sbv3-example1
            cd sbv3-example1
        else
            cd sbv3-example1
            git pull origin
        fi
        printf '\n\nRunning sbv3-example1 build script ...\n'
        bash build.sh "$img_name"
        printf '\n\nStarting docker container ...\n'
        docker run -it -d --rm $img_name
EOF
}

function azure_write_mount_script() {
    local remote_host="$1"
    local remote_user="${2}"
    local local_directory_string="$3"
    if [ -z "$local_directory_string" ]; then
        echo -e "${Red}Local directory was not provided${Color_Off}"
        return 1
    fi

    script_name="mount.sh"
    cat > "$script_name" << EOF
#!/bin/bash

WORKSPACE_DIR="\$HOME"/workspace

[ -d "\$WORKSPACE_DIR" ] && mountpoint -q "\$WORKSPACE_DIR" && { echo "Directory already mounted at \$WORKSPACE_DIR"; exit 0; } || true
[ -d "\$WORKSPACE_DIR" ] && rm -rf "\$WORKSPACE_DIR"

mkdir -p "\$WORKSPACE_DIR"

set -euo pipefail

source /etc/profile

sshfs \
    -o NoHostAuthenticationForLocalhost=yes \
    -o transform_symlinks \
    -o idmap=user \
    -p 10000 "$(whoami)"@localhost:"$local_directory_string" "\$WORKSPACE_DIR"
cd "\$WORKSPACE_DIR"
EOF

    # Make the script executable
    chmod +x "$script_name"

    # Copy the script to the remote host using scp
    scp -q -o LogLevel=QUIET "$script_name" "$remote_user@$remote_host:/home/$remote_user/$script_name"

    # Remove the local script file
    rm "$script_name"
}

function azure_delete_unused_networks() {
    local resource_group="${1:-Default}"
    unused_nics=$(az network nic list --resource-group "$resource_group" --query "[?virtualMachine==null].name" --output tsv)
    echo "$unused_nics" | while read -r line; do
        echo "Deleting NIC: ‘${line}’"
        az network nic delete --resource-group "$resource_group" --name  "$line" &
    done
}
