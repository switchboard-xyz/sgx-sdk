<div align="center">
  <a href="#">
    <img height="170" src="https://github.com/switchboard-xyz/sbv2-core/raw/main/website/static/img/icons/switchboard/avatar.svg" />
  </a>

  <h1>Switchboard SGX SDK</h1>

  <p>SDK for working with Switchboard's SGX and serverless functions.</p>

  <p>
    <a href="https://discord.gg/switchboardxyz">
      <img alt="Discord" src="https://img.shields.io/discord/841525135311634443?color=blueviolet&logo=discord&logoColor=white">
    </a>
    <a href="https://twitter.com/switchboardxyz">
      <img alt="Twitter" src="https://img.shields.io/twitter/follow/switchboardxyz?label=Follow+Switchboard" />
    </a>
  </p>

  <h4>
    <strong>Documentation: </strong><a href="https://docs.switchboard.xyz">docs.switchboard.xyz</a>
  </h4>
</div>

> **Warning** <br /> <strong>This repo is in active development!</strong> Expect breaking changes as the SGX SDK is developed.

## Setup

```bash
pnpm install
pnpm build
cargo build
```

> **Note** <br /> This repo includes dependencies that require an SGX compatible machine to compile.

### Create SGX Virtual Machine

You will need a host machine with SGX in order to develop Switchboard functions.

We will be using Azure, in the future more cloud providers will be supported.

- [Azure CLI Install](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli)

```bash
./scripts/create_azure_vm.sh \
  -n my-sgx-vm
```

This will take a few minutes to provision the machine and install the dependencies.

### Use SGX Virtual Machine

Now attach to the VM and mount the project directory

```bash
./scripts/attach_azure_vm.sh.sh \
  -n my-sgx-vm
```

### Using VS Code Remote SSH Extension

You can attach your VS Code workspace to our virtual machine in Azure. Check out their guide for more infor [VS Code - Remote Development using SSH](https://code.visualstudio.com/docs/remote/ssh).

- Install the **Remote - SSH** extension in VS Code
- Click the green arrows in the bottom left corner to pull up the remote development options
- Select _Connect Host_
- You should see your Azure VM name populated in the lists of hosts. If not you may need to add your host to `~/.ssh/known_hosts` or manually input `azureuser@YOUR_VM_IP_HERE`
- Done! VS Code should now start a new workspace on the remote VM.

Optionally, you can open your VS Code User settings.json and add the following lines to install a set of extensions on your remote host when you connect:

```json
{
  "[rust]": {
    "editor.defaultFormatter": "statiolake.vscode-rustfmt"
  },
  "remote.SSH.defaultExtensions": [
    // Better TOML
    "bungcip.better-toml",
    // Rust Bundle
    "1yib.rust-bundle",
    // vscode-rustfmt
    "statiolake.vscode-rustfmt",
    // JS/TS Bundle
    "ms-vscode.vscode-typescript-next",
    // ESLint
    "dbaeumer.vscode-eslint",
    // Prettier
    "esbenp.prettier-vscode",
    // EditorConfig for VS Code
    "editorconfig.editorconfig",
    // Sync-Rsync (Sync between local and remote workstation)
    "vscode-ext.sync-rsync"
  ]
}
```

## Troubleshooting

1. File a
   [GitHub Issue](https://github.com/switchboard-xyz/sgx-sdk/issues/new)
2. Ask a question in
   [Discord #dev-support](https://discord.com/channels/841525135311634443/984343400377647144)
