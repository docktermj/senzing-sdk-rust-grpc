//! Helper utilities for the Senzing gRPC SDK.
//!
//! This module provides utilities for:
//! - Converting gRPC errors to Senzing errors
//! - Setting up gRPC transport credentials (TLS/mTLS)
//! - Common constants used throughout the SDK
//! - Test utilities

mod constants;
mod error;
#[cfg(test)]
pub mod json;
pub mod runtime;
mod transport_credentials;

pub use constants::*;
pub use error::*;
pub use runtime::*;
pub use transport_credentials::*;
