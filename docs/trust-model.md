# Trust model

Status: design requirements, not implemented guarantees.

## Independent trust domains

Apple ARI, independent camera captures, and operator verification receipts must
have distinct profile identifiers and trust namespaces. A root configured for
one purpose must not authorize another. Adding a development key must never
broaden Apple trust or override a failed Apple evaluation.

Private deployments can enroll a key or private issuing CA through an explicit,
authenticated administrative action. An uploaded self-signed certificate is
untrusted input, not enrollment. Record the trust policy and snapshot used.
Manufacturer credentials require equivalent deliberate trust configuration;
a manufacturer name in metadata establishes nothing by itself.

## Evidence dimensions

| Dimension | Required distinction |
| --- | --- |
| Signature | Does an approved algorithm bind these exact bytes and assertions? |
| Signer trust | Is this signer authorized by the selected policy for this profile? |
| Key protection | Is non-exportability attested, merely asserted, or unknown? |
| Sensor origin | What evidence binds the sensor output to the signing operation? |
| Processing | What evidence binds capture inputs to the rendered output? |
| Time | Is this a device assertion, trusted timestamp, or supported capture interval? |
| Revocation | Which authenticated, sufficiently fresh snapshot was evaluated? |

Do not collapse these dimensions into an unqualified assurance tier. A protected
key may still sign attacker-chosen bytes. A signed firmware version string does
not establish a measured or verified execution state. Authentic captures may
depict screens or staged scenes.

## Freshness and time

A challenge can establish freshness of a response when properly bound, but does
not establish a new sensor capture if software can attach it to old pixels.
A timestamp over a digest establishes an upper bound on existence under its
trust assumptions, not the exact exposure time. Stronger time claims require
reviewed capture-path binding and appropriate trusted time evidence.
Design clock rollback, replay, monotonic counters, offline operation, and reset
behavior explicitly. Do not claim stronger time guarantees when evidence is absent.

## Privacy

Stable per-device public keys can link images. Minimize device identifiers and
precise time/location disclosure. Choose identity and key-lifecycle requirements
before promising anonymity. Rotation alone does not establish unlinkability.
Capture and verification are local by default; remote timestamp or signing
services require explicit configuration and a documented disclosure model.
