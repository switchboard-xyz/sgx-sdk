# V3 Queue Workflow

## Accounts

### ServiceQueueAccount

```rust
pub struct ServiceQueueAccountData {
    // Authority controls adding/removing allowed enclave measurements
    pub authority: Pubkey,
    // All attestation queues will have verifier_queue == self
    pub verifier_queue: Pubkey,
    // allowed enclave measurements
    pub mr_enclaves: [[u8;32]; 32],
    pub mr_enclaves_len: u32,
    pub data: [Pubkey; 32],
    pub data_len: u32,
    // Allow authority to force add a node after X seconds with no heartbeat
    pub allow_authority_override_after: i64,
    // Even if a heartbeating machine quote verifies with proper measurement,
    // require authority signoff.
    pub require_authority_heartbeat_permission: bool,
    pub require_usage_permissions: bool,
    pub max_quote_verification_age: i64,
    pub reward: u32, //TODO
    pub last_heartbeat: i64,
    pub node_timeout: i64,
    pub curr_idx: u32,
    pub gc_idx: u32,
    pub _ebuf: [u8; 1024],
}
```

### NodeAccount

```rust
pub struct NodeAccountData {
    pub authority: Pubkey,
    pub queue: Pubkey,
    pub last_heartbeat: i64,
    pub num_in_use: u32,
    pub verified_at: i64,
    pub valid_until: i64,
    pub _ebuf: [u8; 256],
}
```

### QuoteAccount

```rust
pub struct QuoteAccountData {
    pub node: Pubkey,
    pub node_authority: Pubkey,
    pub queue: Pubkey,
    pub quote_buffer: [u8; 8192],
    pub quote_len: u32,
    pub is_ready: bool,
    pub verification_status: u8,
    pub verification_timestamp: i64,
    pub valid_until: i64,
    pub _ebuf: [u8; 1024],
}
```

## Relations

- ServiceQueue 1:1 VerifierQueue (verifies attestations)
- ServiceQueue 1:N NodeAccounts (performs computations)
- NodeAccounts 1:1 PermissionAccount (access control)
- NodeAccount 1:N QuoteAccount (each 'release' will have its own measurement and associated quote)

## Requirements

- after quote_verify, quote is valid for `service_queue.max_quote_verification_age` seconds
- a service queue must have a dedicated `verifier_queue` to verify attestations
- NodeAccount is required to heartbeat every `service_queue.last_heartbeat` seconds. this is to allow removed MrEnclaves to invalidate a QuoteAccount

## Lifecycle

### Create ServiceQueueAccount

Service queue verifies NodeAccount permissions

### Add MrEnclave

Service queue adds an acceptable measurement representing a valid SGX enclave that is allowed to operate on this queue

### Create NodeAccount & PermissionAccount

Create the node account on the service queue

### Enable Permissions

If neccessary, enable node account permissions

### On-Chain, NodeAccount Creates a QuoteAccount

QuoteAccount is generated from the SGX enclave for a given NodeAccount

Emits `QuoteVerifyRequestEvent`

### Quote Verification Oracle Verifies QuoteAccount's MrEnclave Measurement

Quote verification oracle (Switchboard Attestation Program) is a special oracle that watches and verifies mr enclave measurements.

### NodeAccount is Permitted to Heartbeat with a given QuoteAccount

Heartbeat verifies the node's SGX measurement is still active in the ServiceQueues allowable measurements.

## ChatGPT Summary

The Switchboard Attestation Program involves a workflow that includes several data structures and account relationships. The main components in this workflow include ServiceQueueAccount, NodeAccount, and QuoteAccount. These three account structures are used to manage the attestation process and ensure that the SGX enclaves are operating securely and trustworthily.

ServiceQueueAccount is responsible for managing the allowed enclave measurements, while NodeAccount connects to the service queue and is associated with PermissionAccount for access control. NodeAccount also has a 1:N relationship with QuoteAccount, where each 'release' has its unique measurement and associated quote. A dedicated VerifierQueue is required to verify the attestations associated with a ServiceQueue.

The lifecycle of this workflow consists of several steps: creating a ServiceQueueAccount, adding an acceptable MrEnclave measurement, creating a NodeAccount and PermissionAccount, enabling permissions if necessary, creating a QuoteAccount on-chain, verifying the QuoteAccount's MrEnclave measurement, and allowing the NodeAccount to heartbeat with a given QuoteAccount.

The QuoteAccount is generated from the SGX enclave for a given NodeAccount and emits a QuoteVerifyRequestEvent. The Switchboard Attestation Program, a special oracle, verifies the MrEnclave measurements, ensuring that the NodeAccount's SGX measurement is still active in the ServiceQueue's allowable measurements.

The Switchboard Attestation Program is designed to provide a secure and reliable workflow for managing SGX enclaves, attestation, and ensuring the trustworthiness of the oracle applications. Through this process, developers can confidently leverage secure execution environments for various use cases and build more robust and secure decentralized applications.
