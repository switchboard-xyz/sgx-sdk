# Switchboard Attestation Program: A Comprehensive Guide

- [Switchboard Attestation Program: A Comprehensive Guide](#switchboard-attestation-program-a-comprehensive-guide)
  - [Introduction to the Switchboard Attestation Program](#introduction-to-the-switchboard-attestation-program)
  - [Understanding SGX Attestation](#understanding-sgx-attestation)
  - [SGX Runtime](#sgx-runtime)
  - [Key Components in the Attestation Process](#key-components-in-the-attestation-process)
  - [The Attestation Lifecycle](#the-attestation-lifecycle)
  - [Frequently Asked Questions](#frequently-asked-questions)
  - [Frequently Asked Questions](#frequently-asked-questions-1)
    - [Role of VerifierQueue in the attestation process](#role-of-verifierqueue-in-the-attestation-process)
    - [Adding, updating, and removing MrEnclave measurements](#adding-updating-and-removing-mrenclave-measurements)
    - [Understanding PermissionAccount and its role in access control](#understanding-permissionaccount-and-its-role-in-access-control)
    - [How SGX enclaves generate attestation data (quote) for QuoteAccount](#how-sgx-enclaves-generate-attestation-data-quote-for-quoteaccount)
    - [Switchboard Attestation Program as a special oracle](#switchboard-attestation-program-as-a-special-oracle)
    - [Heartbeat mechanism and maintaining an active SGX enclave](#heartbeat-mechanism-and-maintaining-an-active-sgx-enclave)

## Introduction to the Switchboard Attestation Program

The Switchboard Attestation Program is a Solana contract that verifies attestation quotes on-chain to grant or revoke oracle permissions. Switchboard oracles are required to submit a quote on-chain every 7 days (or configurable by queue), and if not they will have their permissions revoked. The queue manages a set of valid MrEnclave measurements representing

The Switchboard Attestation Program is a vital component of the Switchboard V3 ecosystem, aiming to enhance the security, trust, and decentralization of oracle services provided by Switchboard. The program leverages the power of Trusted Execution Environments (TEEs) to create a secure, verifiable, and trustless environment for oracles to operate within.

In previous versions, Switchboard's security model relied on economic incentives, where data reporters were rewarded or penalized based on their proximity to the median of all reported data. Oracle operators were required to lock up a portion of economic stake and obtain explicit operating approval from the oracle queue's authority. While this model provided a certain level of decentralization and security, it faced limitations in aligning the incentives of oracles with the supported DeFi protocols.

The Switchboard Attestation Program in V3 addresses these limitations by utilizing TEEs. TEEs enable the creation of unique digital signatures for the software running within them and the data generated from that software. This makes it possible to securely and verifiably confirm the integrity and authenticity of the oracles and their executions.

With the Attestation Program, Switchboard V3 allows for:

- **Oracle Verification**: Oracles are encapsulated within TEEs, generating a unique digital signature that can be posted on-chain for public verification. This ensures that the oracles can be trusted while maintaining a decentralized and verifiable system.
- **Execution Verification**: As the oracles participate in a queue and handle user requests, the executions of data feeds, functions, and other tasks can be verifiably carried out. This allows users to confirm that the transactions being sent are strictly dictated by the code that created them, ensuring the integrity and security of the system.

## Understanding SGX Attestation

Intel Software Guard Extensions (SGX) provides a secure execution environment called an enclave, where sensitive code and data are protected from unauthorized access or tampering. The attestation process verifies the enclave's identity and validates its code and data, ensuring authenticity and integrity.

The Switchboard Attestation Program leverages SGX attestation to establish trust in oracle applications, offering a robust mechanism for verifying secure code execution within the Trusted Execution Environment (TEE).

## SGX Runtime

Switchboard services use Gramine to provide the runtime. Gramine enables SGX enclaves to function as if they were running on a standard operating system, handling system calls and other essential interactions.

## Key Components in the Attestation Process

The Switchboard attestation process involves several components that work together to facilitate attestation and verification of SGX enclaves:

- **ServiceQueueAccount**: Manages the list of allowed enclave measurements (MrEnclaves) and maintains a dedicated VerifierQueue to verify attestations.
- **NodeAccount**: Represents an individual SGX enclave, connects to the ServiceQueueAccount, and has a 1:N relationship with QuoteAccount.
- **QuoteAccount**: Stores the attestation data (quote) generated by the SGX enclave for a given NodeAccount.

## The Attestation Lifecycle

The Switchboard Attestation Program follows a structured lifecycle to ensure the secure and trustworthy execution of code within the TEE:

1. Initialize the ServiceQueueAccount
2. Register permitted MrEnclave measurements
3. Create NodeAccount and PermissionAccount
4. Run an SGX Switchboard function, creates QuoteAccount from compiled binary
5. QuoteAccount emits a request to the VerifierQueue associated with the ServiceQueue to verify its MrEnclave measurement
6. The VerifierQueue verifies the MrEnclave measurement on-chain (attestation)
7. The PermissionAccount associated with the NodeAccount has the permissions enabled
8. Maintain an active and trustworthy SGX enclave through heartbeats

By adhering to this lifecycle, the Switchboard Attestation Program ensures the security, authenticity, and trustworthiness of SGX enclaves operating within the TEE.

## Frequently Asked Questions

Here, we address some common questions and provide in-depth explanations to help you better understand the Switchboard Attestation Program and its role in decentralized applications.

1. [Role of VerifierQueue in the attestation process](#faq-1)
2. [Adding, updating, and removing MrEnclave measurements](#faq-2)
3. [Understanding PermissionAccount and its role in access control](#faq-3)
4. [How SGX enclaves generate attestation data (quote) for QuoteAccount](#faq-4)
5. [Switchboard Attestation Program as a special oracle](#faq-5)
6. [Heartbeat mechanism and maintaining an active SGX enclave](#faq-6)

Please refer to the provided questions and answers to gain a deeper understanding of the Switchboard Attestation Program and its inner workings. This comprehensive guide aims to help you build secure and reliable decentralized applications by leveraging the SGX technology and attestation process.

## Frequently Asked Questions

<a name="faq-1"></a>

### Role of VerifierQueue in the attestation process

The VerifierQueue is a set of on-chain oracles that watch the chain for events from QuoteAccounts to verify their MrEnclave measurements. When verified on-chain, the associated PermissionAccounts' permission flags are enabled, indicating they have permission to operate on the queue and be delegated work or compute requests.

<a name="faq-2"></a>

### Adding, updating, and removing MrEnclave measurements

The ServiceQueueAccount's `authority` field is allowed to make changes to the permitted enclave measurements. This could be a DAO-controlled account where participants vote on new trusted code to operate on their program or dApp.

<a name="faq-3"></a>

### Understanding PermissionAccount and its role in access control

The PermissionAccount is a program-derived address between a ServiceQueueAccount and NodeAccount. When a NodeAccount attempts to operate on a ServiceQueueAccount, the PermissionAccount is loaded and its settings are verified against the ServiceQueueAccount settings.

<a name="faq-4"></a>

### How SGX enclaves generate attestation data (quote) for QuoteAccount

The QuoteAccount keypair is generated during the initial execution of the code. The quote is generated from the application binary running within the enclave. The oracles use the Gramine runtime, which provides the abstraction to run the application.

<a name="faq-5"></a>

### Switchboard Attestation Program as a special oracle

The attestation oracles verify the signed measurement against the queue's accepted MrEnclaves and invoke an on-chain transaction, which performs the account checks. If the measurement is invalid, the NodeAccount's associated PermissionAccount's permissions are revoked, and the Node is blocked from performing any access-controlled actions like heartbeating or being delegated work.

<a name="faq-6"></a>

### Heartbeat mechanism and maintaining an active SGX enclave

A NodeAccount is required to heartbeat before the `service_queue.last_heartbeat`. If they do not, they are removed from the queue, and their permissions are revoked, meaning they will not be assigned any future work until they heartbeat again. A NodeAccount is also required to have its MrEnclave measurement before the `service_queue.max_quote_verification_age`.
