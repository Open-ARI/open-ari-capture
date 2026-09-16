# Roadmap

This plan is separate from the Apple ARI implementation roadmap. No dates or
supported profiles are promised. Milestones depend on reviewed evidence.

| Milestone | Work | Exit criteria |
| --- | --- | --- |
| M0: design and evidence | Trust model, C2PA/sidecar decision, versioning, bounded contracts | Reviewed profile proposal and independent expectations; no premature APIs |
| M1: software capture experiment | Capture builder, development signer, offline verifier, Pi adapter | Honest software-asserted evidence; tamper/trust/replay tests; reproducible Pi setup |
| M2: hardware and manufacturer research | Protected signers, boot/capture boundary, provisioning and recovery | Hardware claims backed by tested evidence; documented residual threats |
| M3: interoperability and performance | CLI/FFI/bindings, corpus, packaging, ARM benchmarks | Verification-only dependency isolation, backend parity, installed-package tests |
| M4: release review | Independent review, fixes, documentation and artifact integrity | Approved release scope, conformance evidence, support policy and audit results |

## Work ordering

Approve the trust and evidence contracts before implementing signing or parsing.
Review packaging before choosing a serialization or crypto dependency. Develop
the builder and verifier against independent fixtures, not just self-generated
round trips. The Pi example depends on both and must expose software-only limits.

Hardware research can inform the profile proposal early, but no stronger claim
ships until its capture-path assumptions and adversarial tests are reviewed.
Bindings and optional integration into the existing openari CLI follow native
contracts. Video requires separate ordering, replay, and synchronization design.

## Release gates

Keep `publish = false` until a reviewed release changes it deliberately. Require
independent protocol/security review, adversarial and compatibility fixtures,
parser fuzzing, resource limits, dependency/license review, reproducible platform
checks, signed artifacts and checksums, SBOMs, and documented support boundaries.
Protect release credentials and use narrowly scoped publishing permissions.

Do not treat successful CI, a working Pi demo, or C2PA packaging as an audit or
proof of protected sensor capture. Public claims must match the supported profile
and tested hardware configuration.
