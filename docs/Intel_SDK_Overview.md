# Intel SGX SDK Overview

The following is a high level summary of some pieces of the Intel SGX SDK to assist
in development and building a development environment.

## Intel DCAP (Data Center Attestation Primitives)

Intel DCAP (Data Center Attestation Primitives) is a suite of software tools and
libraries designed to provide attestation services for Intel Software Guard
Extensions (SGX) enabled applications in data center and cloud environments. It
is an alternative to the Intel Enhanced Privacy ID (EPID)-based attestation,
which is primarily used for client devices. DCAP is designed specifically for
data centers and cloud providers, offering a more flexible and scalable
attestation solution.

Intel SGX is a set of hardware and software technologies that aim to protect
sensitive data by creating secure enclaves within the processor. Attestation is
an essential aspect of SGX, as it allows external entities to verify the
integrity of an enclave and its software. In essence, attestation proves that
the code running inside an enclave is authentic and has not been tampered with.

DCAP fits into the SGX attestation process as follows:

- An SGX-enabled application generates a quote containing information about the
  enclave, such as the measurement of the code running within it, and signs the
  quote using the enclave's private key.
- The quote is sent to a relying party, which needs to verify its authenticity.
- The relying party uses the Intel DCAP libraries to perform quote verification.
  The Intel DCAP interacts with the Intel Provisioning Certification Service
  (PCCS), which is a separate service that provides a platform's attestation
  signing key and corresponding certificate chain. PCCS is responsible for
  providing the necessary data to the relying party, allowing them to verify the
  platform's identity during the quote verification process.

You may want to run Intel DCAP for the following reasons:

- **Scalability**: DCAP is designed for large-scale data center and cloud
  environments, making it more suitable for these applications than EPID-based
  attestation.
- **Flexibility**: DCAP allows for customizable attestation service policies,
  giving organizations greater control over how they handle attestation within
  their environment.
- **Simplified Infrastructure**: DCAP reduces the reliance on Intel-managed
  services for attestation, allowing organizations to build their own
  attestation services based on Intel's primitives. By using Intel DCAP, you can
  effectively perform attestation and quote verification for SGX-enabled
  applications in data center and cloud environments while maintaining control
  over your attestation policies and infrastructure.

## Intel PCCS (Provisioning Certificate Caching Service)

The Provisioning Certificate Caching Service (PCCS) is a lightweight service
implemented in Node.js that retrieves and caches PCK Certificates and other
collaterals on-demand at runtime. It interacts with Intel's Provisioning
Certificate Service and exposes a similar HTTPS interface.

In the context of SGX and attestation, PCCS serves the following purposes:

1. Caching Platform Certificates: PCCS caches platform certificates that are
   issued by the Intel Attestation Service (IAS) to reduce latency and improve
   performance during the attestation process. When an attestation request is
   made, PCCS can provide the cached platform certificate, so the relying party
   doesn't need to request it from IAS directly.
2. Providing Attestation Signing Keys and Certificate Chain: PCCS provides the
   attestation signing key and its corresponding certificate chain that is
   required to verify a quote. The quote is a signed statement containing
   information about the enclave (e.g., measurement of the code running within
   it). The relying party needs these keys and certificates to verify the
   authenticity and integrity of the quote during the quote verification
   process.
3. Offloading Intel Attestation Service (IAS) Dependency: By caching platform
   certificates and providing attestation signing keys and certificates, PCCS
   offloads some of the reliance on Intel-managed services for attestation. This
   can help organizations maintain better control over their attestation
   infrastructure and reduce the need for frequent communication with Intel
   services.

PCCS is used in conjunction with Intel DCAP to provide an efficient, scalable,
and flexible attestation solution for SGX-enabled applications in data center
and cloud environments. It simplifies the attestation process by caching
platform certificates, providing attestation signing keys and certificate
chains, and reducing the dependency on Intel-managed services.
