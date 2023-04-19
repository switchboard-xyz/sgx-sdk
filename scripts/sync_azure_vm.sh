#!/bin/bash

set -eo pipefail
stty sane
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/azure_utils.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/ssh_utils.sh"

function display_help {
  printf "\nDescription:\nSend files to Azure virtual machine\n\nUsage:\n%s [-n, vm name] [-d, local dir to mount] [-r, the remote dir] [-s, the ssh key to use]\n\nOptions:\n" "$0"
  echo "-n vm_name, the name of the virual machine in Azure (defaults to my_sgx_machine)"
  echo "-d dir, the local directory to mount on the remote workstation (defaults to current working directory)"
  echo "-r remote_dir, the remote directory to send files (defaults to /home/azureuser/workspace)"
  echo "-s ssh_key, the name of the SSH key to use (defaults to id_azure_rsa)"
  printf "\n\nExample:\n\t%s -n my-vm -d examples/solana-event-watcher\n" "$0"
}

trap 'echo "Error occurred. Displaying help..."; display_help; exit 1' ERR

AZURE_VM_ADMIN_USER="azureuser"
AZURE_SSH_KEY="id_azure_rsa"
REMOTE_DIR="/home/$AZURE_VM_ADMIN_USER/workspace"
LOCAL_DIR="$(get_project_root)"

# Use $AZURE_VM_NAME if provided by shell
if [ -z "$AZURE_VM_NAME" ]; then
  AZURE_VM_NAME="my_sgx_machine"
fi

while getopts 'sd:r:n:s:l:' OPTION; do
  case "$OPTION" in
    d)
      LOCAL_DIR="$OPTARG"
      ;;
    r)
      REMOTE_DIR="$OPTARG"
      ;;
    n)
      AZURE_VM_NAME="$OPTARG"
      ;;
    s)
      AZURE_SSH_KEY="$OPTARG"
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

ssh_sync_files "$AZURE_VM_IP" "$AZURE_VM_ADMIN_USER" "$REMOTE_DIR" "$LOCAL_DIR"

echo -e "${Green}✓ Finished syncing files between local and remote host${Color_Off}"
