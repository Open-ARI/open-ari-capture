# Sources and open decisions

Reviewed on 2026-09-16. External products and specifications can change; pin the
actual specification revisions and implementation versions used by each profile.

- [Apple Reference Image announcement](https://security.apple.com/blog/apple-reference-image)
  describes sensor certification, secure capture, time evidence, and PCC development.
  These are Apple platform guarantees, not properties supplied by this SDK.
- [C2PA implementation guidance 2.3](https://spec.c2pa.org/specifications/specifications/2.3/guidance/Guidance.html)
  separates assertions from trust in the signer. Evaluate the current applicable
  specification and hardware-attestation work before choosing a capture profile.
- [C2PA signing tools](https://opensource.contentauthenticity.org/docs/signing/local-signing/)
  provide an existing route to manifest construction and signing. Reuse is a
  candidate, not an implemented dependency or conformance claim.
- [Raspberry Pi security architecture](https://www.raspberrypi.com/documentation/security/security-architecture.html)
  describes boot and storage protections. These do not establish a protected
  capture path for every Pi/camera combination.

Decisions still open: first hardware target, protected capture assumptions,
C2PA versus sidecar packaging, signed ranges and canonical encoding, independent
profile identifiers, algorithm suites, trusted time model, privacy requirements,
revocation delivery, and release support obligations.
