#!/bin/bash

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/azure_utils.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils/ssh_utils.sh"

function display_help {
  printf "\nDescription:\nBash script to spin up an SGX compatible virtual machine in Azure\n\nUsage:\n%s [-n, vm_name] [-k, ssh_key] [-l, location]\n\nOptions:\n" "$0"
  echo "-n vm_name, the name of the virual machine in Azure (defaults to my_sgx_machine)"
  echo "-k ssh_key, the name of the SSH key to use (defaults to id_azure_rsa)"
  echo "-s vm_size, the type of Azure VM to create (defaults to Standard_DC2s_v3)"
  echo "-l location, the location of the azure vm. See 'az account list-locations -o table' (defaults to uksouth)"
  printf "\n\nExample:\n\t%s my-vm my-sgx-function\n" "$0"
}

trap 'echo "Error occurred. Displaying help..."; display_help; exit 1' ERR

set -eo pipefail

AZURE_VM_NAME=""
AZURE_VM_ADMIN_USER="azureuser"
AZURE_SSH_KEY="id_azure_rsa"
AZURE_VM_LOCATION="uksouth"
AZURE_VM_SIZE="Standard_DC2s_v3" # https://learn.microsoft.com/en-us/azure/virtual-machines/dcv3-series
while getopts 'd:n:i:s:k:l:' OPTION; do
  case "$OPTION" in
    d)
      LOCAL_DIR="$OPTARG"
      ;;
    n)
      AZURE_VM_NAME="$OPTARG"
      ;;
    k)
      AZURE_SSH_KEY="$OPTARG"
      ;;
    s)
      AZURE_VM_SIZE="$OPTARG"
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

if [ -z "$AZURE_VM_NAME" ]; then
  echo "You must provide a VM name with '-n MY_VM_NAME'"
  exit 1
fi

echo -e "AZURE_VM_NAME: ${Blue}$AZURE_VM_NAME${Color_Off}"
echo -e "AZURE_SSH_KEY: ${Blue}$AZURE_SSH_KEY${Color_Off}"
echo -e "AZURE_VM_LOCATION: ${Blue}$AZURE_VM_LOCATION${Color_Off}"
echo -e "AZURE_VM_SIZE: ${Blue}$AZURE_VM_SIZE${Color_Off}"
echo ""

azure_verify_cli_installed
verify_jq_installed

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
## Get or create the Azure VM
############################################################
if [ "$(azure_check_vm_exists "$AZURE_VM_NAME")" == "true" ]; then
    echo "Found an existing Azure VM ($AZURE_VM_NAME) ..."
else
    echo "Creating a new Azure VM ($AZURE_VM_NAME) ..."
    azure_create_vm "$AZURE_VM_NAME" "$AZURE_SSH_KEY" "$AZURE_VM_SIZE"
fi

############################################################
## Setup the Azure VM
############################################################
AZURE_VM_IP=$(azure_get_vm_ip "$AZURE_VM_NAME")
echo -e "AZURE_VM_IP: ${Blue}$AZURE_VM_IP${Color_Off}"

echo Run the following command to ssh into the container:
printf '\n\tssh -t -i %s %s@%s\n\n' "$HOME/.ssh/$AZURE_SSH_KEY" "$AZURE_VM_ADMIN_USER" "$AZURE_VM_IP"
echo Or run the following:
printf '\n\t./scripts/attach_azure_vm.sh -n %s\n\n' "$AZURE_VM_NAME"

# # Run the setup_vm_measurement and grep for the MR_ENCLAVE measurement
# MR_ENCLAVE=$(azure_setup_vm_measurement "$AZURE_VM_IP" "$SGX_IMG_NAME" 2>&1 | tee /dev/tty | grep "MR_ENCLAVE:" | cut -d ' ' -f 2)

# if [ -z "$MR_ENCLAVE" ]; then
#     echo -e "${Red}✗ Failed to get the measurement from the virtual machine${Color_Off}"
#     echo "Try running the script again to rebuild the dependencies"
#     printf '\n\t%s %s\n' "$0" "$*"
#     exit 1
# fi

# echo -e "${Green}✓ Finished initializing the SGX Virtual Machine${Color_Off}"
# separator="----------------------"
# printf '%s\nVM_NAME: %s\nIP: %s\nLocation: %s\nImage: %s\nMR_ENCLAVE: %s\n%s\n\n' "$separator" "$AZURE_VM_NAME" "$AZURE_VM_IP" "$AZURE_VM_LOCATION" "$SGX_IMG_NAME" "$MR_ENCLAVE" "$separator"
