//! Helper utilities for the Senzing gRPC SDK.
//!
//! This module provides utilities for:
//! - Converting gRPC errors to Senzing errors
//! - Setting up gRPC transport credentials (TLS/mTLS)
//! - Common constants used throughout the SDK

mod constants;
mod error;
mod transport_credentials;

pub use constants::*;
pub use error::*;
pub use transport_credentials::*;
