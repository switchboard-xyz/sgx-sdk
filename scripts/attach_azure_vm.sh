#!/bin/bash

set -eo pipefail
stty sane
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/azure_utils.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/ssh_utils.sh"

function display_help {
  printf "\nDescription:\nBash script to attach to an existing SGX compatible virtual machine in Azure\n\nUsage:\n%s [-n, vm name] [-d, local dir to mount] [-s, the ssh key to use] [-l, the vm location]\n\nOptions:\n" "$0"
  echo "-n vm_name, the name of the virual machine in Azure (defaults to my_sgx_machine)"
  echo "-d dir, the local directory to mount on the remote workstation (defaults to current working directory)"
  echo "-s ssh_key, the name of the SSH key to use (defaults to id_azure_rsa)"
  echo "-l location, the location of the azure vm. See 'az account list-locations -o table' (defaults to uksouth)"
  printf "\n\nExample:\n\t%s -n my-vm -d examples/solana-event-watcher\n" "$0"
}

trap 'echo "Error occurred. Displaying help..."; display_help; exit 1' ERR

AZURE_VM_ADMIN_USER="azureuser"
AZURE_SSH_KEY="id_azure_rsa"
REMOTE_DIR="/home/$AZURE_VM_ADMIN_USER/workspace"
LOCAL_DIR="$(get_project_root)"
AZURE_VM_NAME="my_sgx_machine"
AZURE_VM_LOCATION="uksouth"
while getopts 'sd:n:s:l:' OPTION; do
  case "$OPTION" in
    d)
      LOCAL_DIR="$OPTARG"
      ;;
    n)
      AZURE_VM_NAME="$OPTARG"
      ;;
    s)
      AZURE_SSH_KEY="$OPTARG"
      ;;
    l)
      AZURE_VM_LOCATION="$OPTARG"
      ;;
    ?)
      display_help
      exit 1
      ;;
  esac
done
shift "$(($OPTIND -1))"

azure_verify_cli_installed
verify_jq_installed
verify_sshfs_installed
verify_rsync_installed

echo -e "AZURE_VM_NAME: ${Blue}$AZURE_VM_NAME${Color_Off}"

############################################################
## Login to Azure and verify the SSH keys are setup
############################################################
AZ_ACCOUNT=$(az account show -o json)
AZ_USER=$(echo "$AZ_ACCOUNT" | jq -r '.user.name')
AZ_ACCOUNT_STATE=$(echo "$AZ_ACCOUNT" | jq -r '.state')
if [[ $AZ_ACCOUNT_STATE != "Enabled" ]]; then
    az login
fi

ssh_setup_keys "$AZ_USER" "$AZURE_SSH_KEY"

############################################################
## Verify the VM Exists
############################################################
AZURE_VM_IP=$(azure_get_vm_ip "$AZURE_VM_NAME")
if [[ "$(azure_check_vm_exists "$AZURE_VM_NAME")" == "false" ]]; then
    echo "An Azure virtual machine with the name $AZURE_VM_NAME was not found"
    exit 1
fi

trap '' ERR

echo Run the following command to ssh into the container:
printf '\n\tssh -t %s@%s\n\n' "$AZURE_VM_ADMIN_USER" "$AZURE_VM_IP"

# ssh "${AZURE_VM_ADMIN_USER}@${AZURE_VM_IP}" "echo \"`cat ~/.ssh/id_azure_rsa.pub`\" >> .ssh/authorized_keys"
# ssh-keygen -t rsa -b 4096 -N "" -f ~/.ssh/id_rsa -C "localhost"

# WORKING!!
# May need to add our ssh key to remote server .ssh/authorized_keys
# May need to add an ssh key to the remote server
# May need to start ssh server for localhost
# reverse_sshfs "$AZURE_VM_IP" 22 "$AZURE_VM_ADMIN_USER" "$LOCAL_DIR"

REMOTE_DIR="/home/$AZURE_VM_ADMIN_USER/workspace"

ssh_from_local_to_remote "$AZURE_VM_IP" "$AZURE_VM_ADMIN_USER" "$REMOTE_DIR" "$LOCAL_DIR"

# Run some commands inside the machine
ssh -X -t -p 22 "$AZURE_VM_ADMIN_USER"@"$AZURE_VM_IP" -R 10000:localhost:22 "cd $REMOTE_DIR; bash; cd;"

# Copy files from remote host to local machine
ssh_from_remote_to_local "$AZURE_VM_IP" "$AZURE_VM_ADMIN_USER" "$REMOTE_DIR" "$LOCAL_DIR"
