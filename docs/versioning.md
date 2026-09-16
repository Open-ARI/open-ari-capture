# Versioning

No independent capture profile exists yet. Library version 0.1.0-dev.0 is a
workspace version and does not imply an evidence format called v1.

| Version or identity | Purpose |
| --- | --- |
| Library semver | Source API and behavior compatibility |
| Capture profile ID and revision | Namespaced wire format, required evidence, and validation rules |
| Algorithm suite | Approved algorithms, parameters, and domain separation for a profile |
| Report schema | Machine-readable results and stable reason codes |
| Policy ID and version | Application requirements and allowed trust domains |
| Trust/revocation generation and digest | Exact external evidence evaluated |
| Device, firmware, and processing identities | Capture-path evidence, not library versions |
| FFI ABI version | Future native interoperability, independently negotiated |
| Conformance corpus version and digest | Reproducible positive and negative expectations |

Keep Apple identifiers outside the independent capture namespace. Do not accept
a generic v1 without identifying the scheme and profile. Bind profile and
algorithm selection into authenticated evidence; reject downgrade attempts.

Capabilities must declare implemented profiles and constraints. Unknown critical
extensions cannot be silently discarded. Preserve unrecognized data for forensic
inspection only within bounds and mark it untrusted.

Record provider identity and versions when available. An unavailable device or
provider version remains unknown; do not fabricate it. Evaluate a manufacturer's
firmware updates and key rotations against explicit compatibility and rollback
policy rather than assuming a newer version is trustworthy.
