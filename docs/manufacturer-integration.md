# Manufacturer integration

Status: proposed integration requirements. No production SDK is available.

The SDK should accept immutable frames and capture metadata through a documented
adapter and delegate signing to the manufacturer's protected backend. It must
not require exporting private keys. Shared code handles evidence construction,
bounded verification, profile negotiation, reports, and conformance fixtures.

The manufacturer must supply and substantiate:

- A capture boundary that explains which sensor, bus, processor, and firmware
  are trusted and which components can replace pixels or metadata.
- Protected key generation, provisioning, device/sensor binding, and certificate
  issuance with key usages and profile constraints.
- Firmware verification, measurements where applicable, update authorization,
  anti-rollback, and the link between execution state and signing authority.
- Reviewed rendering behavior and authenticated RAW/derived-image relationships.
- Revocation, key rotation, compromised-device handling, support lifetime,
  ownership transfer, factory reset, and recovery procedures.
- A privacy design addressing linkability and disclosure of serial numbers,
  precise time/location, and hardware identifiers.

The SDK must not grant a hardware assurance label simply because a backend is
called secure or because a manufacturer signs a certificate. Verification needs
evidence accepted by the relying party's policy.

Prototype performance work should measure frame-to-evidence latency, hashing
throughput, peak memory, signing hardware round trips, and verification latency
on representative ARM hardware. Stream hashes and avoid unnecessary copies
where the chosen profile permits. Hardware acceleration must produce equivalent
results; GPU support requires a measured benefit and a reviewed dependency cost.

Video is a separate profile-design problem. Specify ordering, dropped frames,
segment boundaries, truncation, audio synchronization, and replay before making
video capture claims. Still-image signing is not automatically a video protocol.
