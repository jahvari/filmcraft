//! Installed Intel VPL identity/session integration, Windows x64 MSVC.
//!
//! The declaration boundary alone does not load a driver or advertise codec support.

#[allow(unsafe_code)] // Raw public OS media API declarations only (ADR 0001).
mod ffi;

#[cfg(test)]
#[allow(unsafe_code)] // FFI layout/union contract tests; every union read is justified locally.
mod abi_tests;
