# Contributor instructions

- Read README.md and the relevant design documents before editing.
- This is an architecture-only scaffold. Do not imply implemented signing, verification, C2PA conformance, or sensor attestation.
- Independent capture evidence is not Apple ARI. Never share trust roots or silently fall back between those profiles.
- Distinguish valid signatures, trusted signers, key protection, sensor origin, processing integrity, and time evidence.
- A hardware-held key is not proof of a protected capture path. Never upgrade assurance from a signer-supplied label alone.
- Keep verification deterministic, offline, bounded, and independent of capture drivers and private keys.
- Keep capture profiles, library versions, ABI, report schemas, policies, trust snapshots, and firmware identities separate.
- Keep the existing openari CLI as the optional integration point; do not create a competing executable.
- Use the approved logo and centered README header pattern from shoon/fv-ssh-unlock, including license, docs, sponsor, and applicable status badges.
- Use plain factual writing without em dashes or AI filler. Do not generate artwork unless requested.
- Use Apache-2.0 SPDX source headers, Copyright 2026 ncdents, LLC., and DCO sign-off.
- Run relevant CI checks. Add dependencies only for reviewed implementation needs.
- Do not publish packages or claim production readiness without the release gates in docs/roadmap.md.
