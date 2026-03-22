//! # SafeIO
//!
//! Deterministic async runtime for safety-critical systems.
//!
//! SafeIO provides a uniform async API for applications running on
//! certified operating systems (QNX, SafeRTOS, Linux).
//!
//! **Status:** This crate is in early design. The API is not yet available.
//!
//! Part of the [DriftSys](https://github.com/driftsys) ecosystem.

#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]
