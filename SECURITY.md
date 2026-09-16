# Security policy

## Supported versions

OpenARI Capture is an architecture-only, pre-1.0 scaffold. No capture or
verification profile is implemented, and no external audit has occurred.
Security fixes target main and the latest release when releases exist.
Older development snapshots are not maintained.

## Report a vulnerability

Use [private vulnerability reporting](https://github.com/open-ari/openari-capture/security/advisories/new).
Include the affected commit, reproduction, expected impact, and disclosure
constraints. Do not open a public issue for suspected vulnerabilities.
Use synthetic samples. Do not attach private photographs, device identities,
provisioning secrets, or keys without agreeing on safe handling.

We aim to acknowledge reports within seven days. This is a maintainer response
target, not a service guarantee. Allow time for investigation and a coordinated fix.

See [the threat model](docs/threat-model.md). A valid capture signature is not
automatically evidence of sensor origin or a truthful scene.
