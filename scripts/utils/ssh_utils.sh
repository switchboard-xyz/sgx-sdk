#!/bin/bash

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/utils.sh"

# https://code.visualstudio.com/docs/remote/troubleshooting#_using-sshfs-to-access-files-on-your-remote-host

function verify_sshfs_installed() {
    local is_homebrew_installed=false
    [[ "$(uname)" == "Darwin" && "$(command -v brew)" ]] && is_homebrew_installed=true

    if ! command -v sshfs &> /dev/null; then
        if $is_homebrew_installed; then
            brew update
            brew install --cask macfuse
            brew install gromgit/fuse/sshfs-mac
            brew link --overwrite sshfs-mac
        else
            install_dependency sshfs
        fi
    fi
}

function verify_rsync_installed() {
    install_dependency rsync
}

# If needed, create an SSH key and add to the ssh-agent
function ssh_setup_keys() {
    local user="${1:-ssh_user}"
    local key_name="${2:-id_azure_rsa}"
    local key_path="$HOME/.ssh/$key_name"

    if [ ! -f "$key_path" ]; then
        ssh-keygen -t rsa -b 4096 -C "$user" -f "$key_path" -q -N ""
        echo -e "${Purple}SSH key generated at $key_path ${Color_Off}"

        # Check if ssh-agent is running
        if [ -z "$SSH_AUTH_SOCK" ] || ! ssh-add -l >/dev/null 2>&1; then
            eval "$(ssh-agent -s)"
        fi

        # If the host OS is macOS, start the SSH agent if it's not running and add the key to the Apple Keychain
        if [ "$(uname)" == "Darwin" ]; then
            # Add the SSH key to the Apple Keychain
            ssh-add --apple-use-keychain "$key_path"
        else
            ssh-add "$key_path"
        fi
    fi
}

function ssh_add_host_config() {
    local host=${1}
    if [ -z "$host" ]; then
        echo "Need to provide host for the ssh host config"
        exit 1
    fi
    local ip=${2}
    if [ -z "$ip" ]; then
        echo "Need to provide ip for the ssh host config"
        exit 1
    fi

    local remote_user="${3:-azureuser}"
    local key_name="${4:-id_azure_rsa}"

    config_string="Host $host
  HostName $ip
  IdentityFile "~/.ssh/$key_name"
  ForwardX11 yes
  Port 22
  User $remote_user
  RemoteForward 10000 localhost:22
"
    ssh_config="${HOME}/.ssh/config"
    mkdir -p "${HOME}/.ssh" || true
    cat <(echo "$config_string") "$ssh_config" > >(tee "${ssh_config}.tmp") && mv "${ssh_config}.tmp" "$ssh_config" >/dev/null 2>&1
    chmod 600 "$ssh_config" || true
}

function ssh_verify_local_server() {
    if [ "$(uname)" == "Darwin" ]; then
        if [[ "$(sudo systemsetup -getremotelogin | grep 'Remote Login: On')" ]]; then
            return 0;
        else
            echo "Remote login is disabled"
            printf "Run the following command to enable Remote login:\n\t%s\n\t%s\n" "sudo systemsetup -setremotelogin on" "sudo systemsetup -getremotelogin"

        fi
    fi
}

function ssh_from_local_to_remote() {
    local REMOTE_ADDR="$1"
    local REMOTE_USER="$2"
    local REMOTE_DIR="$3"
    local LOCAL_DIR="$4"

    local RSYNC_ARG_STRING="-aurlptz"
    [[ -n "$VERBOSE" ]] && RSYNC_ARG_STRING="-aurlptz --progress --verbose"

    # Copy files from local machine to remote host
    rsync $RSYNC_ARG_STRING --exclude='.git' --exclude='node_modules/' --exclude='target/' --exclude='.turbo/' "$LOCAL_DIR/" "$REMOTE_USER@$REMOTE_ADDR:$REMOTE_DIR/"
}

function ssh_from_remote_to_local() {
    local REMOTE_ADDR="$1"
    local REMOTE_USER="$2"
    local REMOTE_DIR="$3"
    local LOCAL_DIR="$4"

    local RSYNC_ARG_STRING="-aurlptz"
    [[ -n "$VERBOSE" ]] && RSYNC_ARG_STRING="-aurlptz --progress --verbose"

    # Copy files from remote host to local machine
    rsync $RSYNC_ARG_STRING --exclude='.git' --exclude='node_modules/' --exclude='target/' --exclude='.turbo/' "$REMOTE_USER@$REMOTE_ADDR:$REMOTE_DIR/" "$LOCAL_DIR/";
}

function ssh_sync_files() {
    ssh_from_local_to_remote "$AZURE_VM_IP" "$AZURE_VM_ADMIN_USER" "$REMOTE_DIR" "$LOCAL_DIR"
    ssh_from_remote_to_local "$AZURE_VM_IP" "$AZURE_VM_ADMIN_USER" "$REMOTE_DIR" "$LOCAL_DIR"
}

function reverse_sshfs() {
    ##/*
    ## * @Author: AllenYL 
    ## * @Date: 2017-11-08 11:37:31 
    ## * @Last Modified by:   allen7575@gmail.com 
    ## * @Last Modified time: 2017-11-08 11:37:31 
    ## */

    ssh_verify_local_server

    #
    # mount local directory to remote through reverse sshfs
    # 
    # usage:
    #       ./reverse_sshfs.sh [remote_addr] [remote_ssh_port] [remote_user] [local_dir]
    # 
    # [local_dir] is a path relative to this script
    # 
    # This script will automatcally create a directory named "project_$LOCAL_USER" in remote user's home dir,
    # and mount [local_dir] to this point. When exit, will umount "project_$LOCAL_USER" and deleted it.
    # 

    ##
    ## linux - how to mount local directory to remote like sshfs? - Super User 
    ## https://superuser.com/questions/616182/how-to-mount-local-directory-to-remote-like-sshfs
    ##

    local LOCAL_USER=$(whoami)
    local REMOTE_USER="$3"

    local LOCAL_DIR="$4"
    local REMOTE_DIR="/home/$REMOTE_USER/workspace"

    local LOCAL_ADDR="localhost"
    local REMOTE_ADDR="$1"

    local LOCAL_PORT="22"
    local FORWARD_PORT="10000"
    local REMOTE_PORT="$2"

    local LOCAL_SSH="-p $FORWARD_PORT $LOCAL_USER@$LOCAL_ADDR"
    local REMOTE_SSH="-p $REMOTE_PORT $REMOTE_USER@$REMOTE_ADDR"

    # local SSHFS_OPTION="-o NoHostAuthenticationForLocalhost=yes -odebug,sshfs_debug,loglevel=debug"
    local SSHFS_OPTION="-o NoHostAuthenticationForLocalhost=yes"

    ###############
    ## With ssh, how can you run a command on the remote machine without exiting? - Super User 
    ## https://superuser.com/questions/261617/with-ssh-how-can-you-run-a-command-on-the-remote-machine-without-exiting
    ##
    ## Here I use -t to force the allocation of a pseudo-terminal, which is required for an interactive shell. 
    ## Then I execute two commands on the server: first the thing I wanted to do prior to opening the interactive shell 
    ## (in my case, changing directory to a specific folder), and then the interactive shell itself. 
    ## bash sees that it has a pseudo-terminal and responds interactively.
    ##
    ###############
    ## Why does an SSH remote command get fewer environment variables then when run manually? - Stack Overflow 
    ## https://stackoverflow.com/questions/216202/why-does-an-ssh-remote-command-get-fewer-environment-variables-then-when-run-man
    ##
    ## sourcing the profile before running the command
    ## ssh user@host "source /etc/profile; /path/script.sh"
    ##
    ## usage:
    ##      ssh -t -p 88 root@10.1.53.168 -R 10000:localhost:22 \
    ##      "source /etc/profile; sshfs  -p 10000 allenyllee@localhost:/media/allenyllee/Project/Project/server_setup/nvidia_docker/project ./project2;bash"
    ## options:
    ##       -v Verbose 
    ##       -X X11 forwarding
    ##       -t pseudo-terminal for an interactive shell
    ##
    [[ -n $VERBOSE ]] && echo "ssh -X -t $REMOTE_SSH -R $FORWARD_PORT:localhost:$LOCAL_PORT \
        \"source /etc/profile; \
        mkdir $REMOTE_DIR; \
        sshfs $SSHFS_OPTION $LOCAL_SSH:$LOCAL_DIR $REMOTE_DIR; \
        cd $REMOTE_DIR; \
        bash; \
        cd && fusermount -u $REMOTE_DIR && rm -r $REMOTE_DIR;\""
    ssh -X -t $REMOTE_SSH -R $FORWARD_PORT:localhost:$LOCAL_PORT \
        "source /etc/profile; \
        mkdir $REMOTE_DIR; \
        sshfs $SSHFS_OPTION $LOCAL_SSH:$LOCAL_DIR $REMOTE_DIR; \
        cd $REMOTE_DIR; \
        bash; \
        cd && fusermount -u $REMOTE_DIR && rm -r $REMOTE_DIR;"
}
