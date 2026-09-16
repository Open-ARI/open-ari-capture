# Threat model

Status: initial analysis. Protocol and hardware reviews remain required.

Assets include frame integrity, signing keys, trusted policy, device identity,
firmware measurements, evidence availability, and photographer privacy. Model
malicious uploaded packages, compromised capture applications or operating systems,
physical access, malicious peripherals, and compromised issuers separately.

| Threat | Required treatment |
| --- | --- |
| Software substitutes a generated image | Software-only signing must not claim proven sensor origin |
| Secure element becomes a signing oracle | Restrict and attest which capture path can authorize signing |
| Sensor bus injection or sensor replacement | Define the hardware boundary and binding; test substitution |
| Replay with fresh metadata | Bind capture, challenge/counter, and metadata in the protected path |
| Altered RAW-to-JPEG processing | Bind inputs and outputs; distinguish recorded processing from enforced processing |
| Claimed hardware assurance in attacker metadata | Derive assurance only from verified evidence and explicit policy |
| Unknown profile or weaker algorithm | Exact authenticated dispatch; no fallback or downgrade |
| Stale revocation or rollback | Authenticated snapshots, freshness checks, and explicit failure outcomes |
| Key theft or compromised issuer | Provisioning controls, scoped keys, rotation, revocation, and incident response |
| Parser bombs or malformed certificates | Resource budgets, fuzzing, adversarial corpora, and independent review |
| Network URLs in attacker evidence | No implicit fetches; explicit bounded network adapters outside verification |
| Device tracking or sensitive logs | Minimize identifiers; never log images or key material by default |

The Pi experiment must demonstrate that replacing captured pixels before signing
can still produce a valid software signature, and that its verifier does not
promote this to hardware sensor evidence. Document this limitation as an expected
security property of the prototype, not an implementation success at attestation.

Hardware-backed work needs tests against signing arbitrary files, modified
firmware, replay, key migration, and sensor substitution. Secure boot and measured
boot are different properties; neither substitutes for a reviewed capture path.

No profile can establish the truth of a depicted event solely from sensor origin.
