// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 ncdents, LLC.

//! Architecture-only scaffold for independent capture-evidence verification.
//!
//! No parser, signature verification, or trust evaluation is implemented.
//! Future verification must be offline and bounded, with explicit profiles
//! and trust inputs. It must not depend on capture drivers or private keys,
//! and must never interpret an independent capture signature as Apple ARI.
