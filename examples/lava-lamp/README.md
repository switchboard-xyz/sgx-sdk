# Switchboard attestation example

In this example we will create a verifiable, confidential binary through the
Switchboard attestation service.

The steps to create the verifiable binary have been done for you, your only job
is to write the code!

In this setup we have created you simply need to run `bash build.sh` to build
your verifiable image.

You will see a `measurement.txt` generated in your directory which acts as the
signed identifier for your sgx secured program.

On startup of this program, we will generate a secure key using entropy from
within Intel SGX.

Then, we will generate an SGX Quote (https://is.gd/TYLu5k) to associate this key
with this enclave's measurement so you can prove that this generated key was
generated securely and confidentially within the enclave.

After the initialization transactions are completed, you can then use this
signer for privilidged actions in your application!

See https://crates.io/crates/solana_switchboard_attestation_program_sdk for
asserting this chain inside your program.

## Hosting your machine

Dont have an SGX machine? No problem!

Many cloud providers provide sgx machines so you can fire it up and forget.

Run the following to fire up an SGX machine in azure cloud and clone and build
this repo

`bash scripts/sgx_init_machine.sh`

## Notes

### Intel SGX SDK

Source:
[Intel Docs](https://software.intel.com/content/www/us/en/develop/articles/intel-software-guard-extensions-data-center-attestation-primitives-quick-install-guide.html),
[Download](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/linux-overview.html)

Run the following commands to install the `intel-sgx`

```
curl -fsSLo /usr/share/keyrings/intel-sgx-deb.asc https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key
```
