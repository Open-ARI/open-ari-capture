# Sources and open decisions

Reviewed on 2026-10-31. External products and specifications can change; pin the
actual specification revisions and implementation versions used by each profile.

- [Apple Reference Image announcement](https://security.apple.com/blog/apple-reference-image)
  (2026-09-15) describes sensor certification, secure capture, time evidence, and
  PCC development. These are Apple platform guarantees, not properties supplied
  by this SDK. Design points relevant to independent profiles:
  - A per-sensor ECDSA P-256 key generated in the sensor at manufacture and
    certified by a factory CA, plus a separately attested secure-element key,
    bound together by a CA-signed device manifest.
  - The sensor signs pixels together with a digest supplied by the OS, which
    commits to the latest timestamp token and the device and boot manifests.
    OS-derived metadata (zoom, exposure, lens) is signed by the secure element,
    not the sensor.
  - Capture time is an interval: an RFC 3161 token received before capture as
    the lower bound, and a token over a commitment to the capture signature as
    the upper bound. Offline captures get the upper bound later.
  - Raw evidence is kept as a DNG negative. It carries device-unique
    certificates, so negatives taken on one device are linkable. Apple removes
    that linkage only by re-signing a developed image with its own service key.
  - Revocation covers single images and whole sensors and depends on a private
    Apple record.
- [Apple Support 128064](https://support.apple.com/en-us/128064) (2026-09-17)
  notes that undeveloped negatives may include the device serial number.
- [Format status in open-ari-core](https://github.com/open-ari/open-ari-core/blob/main/docs/format-status.md)
  tracks what Apple has and has not published.
- [C2PA implementation guidance 2.3](https://spec.c2pa.org/specifications/specifications/2.3/guidance/Guidance.html)
  separates assertions from trust in the signer. Evaluate the current applicable
  specification and hardware-attestation work before choosing a capture profile.
- [C2PA signing tools](https://opensource.contentauthenticity.org/docs/signing/local-signing/)
  provide an existing route to manifest construction and signing. Reuse is a
  candidate, not an implemented dependency or conformance claim.
- [RFC 3161](https://www.rfc-editor.org/rfc/rfc3161) defines the timestamp tokens
  Apple uses for capture bounds and is a candidate for independent profiles.
- [draft-ietf-lamps-pq-composite-sigs](https://datatracker.ietf.org/doc/draft-ietf-lamps-pq-composite-sigs/)
  defines composite ML-DSA signatures. open-ari-core implements revision 13 for
  ML-DSA-87 with RSA-3072-PSS; a capture profile choosing a composite suite
  must pin the exact revision.
- [Raspberry Pi security architecture](https://www.raspberrypi.com/documentation/security/security-architecture.html)
  describes boot and storage protections. These do not establish a protected
  capture path for every Pi/camera combination.

Decisions still open: first hardware target, protected capture assumptions,
C2PA versus sidecar packaging, signed ranges and canonical encoding, independent
profile identifiers, algorithm suites, trusted time model, privacy requirements,
revocation delivery, and release support obligations.
