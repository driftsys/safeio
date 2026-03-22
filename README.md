# SafeIO

**Deterministic async runtime for safety-critical systems.**

SafeIO provides a uniform async API for applications running on certified operating systems (QNX, SafeRTOS, Linux). It targets the same developer ergonomics as Tokio — tasks, channels, timers, I/O — with a smaller, auditable surface designed for safety-critical contexts.

The runtime features a deterministic work-stealing scheduler designed for formal verification, bounded worst-case execution time, and ASIL-B compliance.

## Status

Early design. Not yet usable. This crate is published to reserve the name.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

Part of the [DriftSys](https://github.com/driftsys) ecosystem.
