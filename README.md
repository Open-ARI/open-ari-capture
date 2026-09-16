<p align="center">
  <a href="https://openari.org"><img src="assets/openari-logo.png" alt="OpenARI mosaic logo" width="128" height="128"></a>
</p>

<h1 align="center">OpenARI Capture</h1>

<p align="center">
  Capture-signing SDK research for independent cameras and embedded devices.
</p>

<p align="center">
  <a href="https://github.com/open-ari/open-ari-capture/actions/workflows/ci.yml"><img src="https://github.com/open-ari/open-ari-capture/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI status"></a>
  <a href="docs/roadmap.md"><img src="https://img.shields.io/badge/status-design%20scaffold-orange" alt="Design scaffold status"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/open-ari/open-ari-capture" alt="Apache 2.0 license"></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Rust-1.88%2B-000000?logo=rust&amp;logoColor=white" alt="Minimum supported Rust version: 1.88"></a>
  <a href="docs/architecture.md"><img src="https://img.shields.io/badge/docs-architecture-007D79" alt="Capture architecture"></a>
  <a href="https://github.com/sponsors/shoon"><img src="https://img.shields.io/badge/Sponsor-shoon-EA4AAA?logo=githubsponsors&amp;logoColor=white" alt="Sponsor shoon on GitHub"></a>
</p>

OpenARI Capture is a separate research track for creating and verifying signed
capture evidence. The intended users are Raspberry Pi developers, camera
manufacturers, and applications that need an explicit record of who signed
which image bytes and what capture guarantees are supported by evidence.

**Current status: architecture-only scaffold.** The Rust workspace builds, but
does not capture frames, sign images, verify signatures, implement C2PA, or attest
sensor origin. There are no supported capture profiles, stable APIs, or published
SDK packages. No security audit has occurred.

This project cannot issue Apple capture attestations. Apple ARI verification
remains in [open-ari-core](https://github.com/open-ari/open-ari-core). Independent
capture profiles use separate identifiers, trust roots, and verification policy.
OpenARI is not affiliated with or endorsed by Apple.

## Intended integration

| Component | Responsibility |
| --- | --- |
| `openari-capture` | Build capture evidence and request signatures through reviewed backends |
| `openari-capture-verify` | Verify independent capture profiles offline with explicit trust policy |
| Camera adapters | Supply captured frames and describe the actual capture boundary |
| Signing backends | Development keys, secure elements, TPMs, or manufacturer hardware |
| Existing `openari` CLI and language bindings | Optional integration after contracts are reviewed |

These are planned boundaries. Only documentation-only Rust crate entry points
exist today. Verification-only consumers must not acquire camera-driver or
private-key dependencies. No second CLI is being introduced.

## Trust and sensor origin

A valid signature binds a key to particular bytes and assertions. It does not,
by itself, prove those bytes originated at a camera sensor. A hardware-held key
also does not prove sensor origin if untrusted software can choose its input.

The first proposed Pi prototype is software-asserted capture. A private deployment
can enroll its own keys explicitly; unknown self-signed credentials do not become
trusted merely because they accompany an image. Stronger manufacturer profiles
need evidence about the protected capture path, firmware, provisioning, and
compromise recovery. Even sensor-origin evidence cannot rule out staged scenes
or photographs of screens.

## Development

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
```

The workspace uses the pinned toolchain in `rust-toolchain.toml`, has no external
Rust dependencies, and disables package publication. Passing scaffold checks
does not establish capture or verification functionality.

## Design and work plan

| Document | Contents |
| --- | --- |
| [Architecture](docs/architecture.md) | Crate boundaries, evidence flow, and existing CLI integration |
| [Trust model](docs/trust-model.md) | Independent trust, assurance evidence, and privacy |
| [Versioning](docs/versioning.md) | Capture profiles, APIs, algorithms, reports, and hardware versions |
| [Raspberry Pi prototype](examples/raspberry-pi/README.md) | Proposed experiment and honest acceptance criteria |
| [Manufacturer integration](docs/manufacturer-integration.md) | Provisioning, protected capture, and deployment requirements |
| [Threat model](docs/threat-model.md) | Substitution, signing-oracle, replay, downgrade, and privacy threats |
| [Roadmap](docs/roadmap.md) | Milestones, dependencies, and release gates |
| [Sources and decisions](docs/sources.md) | External evidence and unresolved design choices |

Track implementation in [issues](https://github.com/open-ari/open-ari-capture/issues)
and [milestones](https://github.com/open-ari/open-ari-capture/milestones).
See [CONTRIBUTING.md](CONTRIBUTING.md) for DCO sign-off and
[SECURITY.md](SECURITY.md) for private vulnerability reporting.

## Support and license

[Sponsor shoon](https://github.com/sponsors/shoon) to support development.

[Apache License 2.0](LICENSE). Copyright 2026 ncdents, LLC.
