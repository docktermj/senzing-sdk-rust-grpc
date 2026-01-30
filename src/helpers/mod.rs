//! Helper utilities for the Senzing gRPC SDK.
//!
//! This module provides utilities for:
//! - Converting gRPC errors to Senzing errors
//! - Setting up gRPC transport credentials (TLS/mTLS)
//! - Common constants used throughout the SDK
//! - Test utilities

mod constants;
pub mod isdestroyed;
#[cfg(test)]
pub mod json;
pub mod runtime;
pub mod short_function_name;
mod transport_credentials;

pub use constants::*;
pub use runtime::*;
pub use transport_credentials::*;
