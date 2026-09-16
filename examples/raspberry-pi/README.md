# Raspberry Pi capture experiment

Status: planned example. This directory contains no executable capture software.

The first experiment will use a documented Pi model, camera module, OS image,
and capture stack. Record those exact versions; do not claim every Pi or camera
is supported. Capture a frame, preserve immutable input bytes, construct evidence
under the reviewed experimental profile, and sign with a development credential.

Verify on a separate machine using an explicitly enrolled development root/key.
Report software-asserted capture, signature and trust outcomes, and unavailable
hardware evidence independently. The result is not Apple ARI.

## Acceptance criteria

- Capture and verify a permitted sample under the chosen profile.
- Tampered bytes fail integrity validation; unknown keys remain untrusted.
- Replaced pixels signed by the same application must not be labeled sensor-attested.
- Document time assertions, challenge/replay limits, metadata handling, and privacy.
- Exercise resource limits and cancellation on the selected hardware.
- Keep development keys out of the repository and logs; use ephemeral test keys
  with clearly scoped test trust if tests need credentials.
- Publish reproducible setup and measurements without promising production assurance.

A later experiment may use a secure element or TPM and verified boot. Those
capabilities improve particular properties but do not, on their own, prove the
sensor-to-signer relationship. Hardware integration has a separate review gate.
