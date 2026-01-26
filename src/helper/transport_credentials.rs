//! gRPC transport credentials utilities.
//!
//! This module provides utilities for configuring gRPC transport credentials
//! with support for:
//! - Insecure connections (no TLS)
//! - Server-side TLS (server certificate verification)
//! - Mutual TLS (mTLS) with client certificates
//!
//! Configuration is done via environment variables:
//! - `SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE`: Path to server CA certificate (enables TLS)
//! - `SENZING_TOOLS_CLIENT_CERTIFICATE_FILE`: Path to client certificate (for mTLS)
//! - `SENZING_TOOLS_CLIENT_KEY_FILE`: Path to client private key (for mTLS)
//! - `SENZING_TOOLS_CLIENT_KEY_PASSPHRASE`: Optional passphrase for encrypted client key

use pkcs8::der::pem::PemLabel;
use pkcs8::der::Decode;
use pkcs8::{EncryptedPrivateKeyInfo, LineEnding, SecretDocument};
use std::env;
use std::fs;
use std::path::Path;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

/// Environment variable for the server CA certificate file path.
pub const ENV_SERVER_CA_CERTIFICATE_FILE: &str = "SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE";

/// Environment variable for the client certificate file path.
pub const ENV_CLIENT_CERTIFICATE_FILE: &str = "SENZING_TOOLS_CLIENT_CERTIFICATE_FILE";

/// Environment variable for the client key file path.
pub const ENV_CLIENT_KEY_FILE: &str = "SENZING_TOOLS_CLIENT_KEY_FILE";

/// Environment variable for the client key passphrase.
pub const ENV_CLIENT_KEY_PASSPHRASE: &str = "SENZING_TOOLS_CLIENT_KEY_PASSPHRASE";

/// Error type for transport credential operations.
#[derive(Debug)]
pub enum TransportCredentialsError {
    /// Failed to read a certificate file.
    CertificateReadError {
        path: String,
        source: std::io::Error,
    },
    /// Failed to read a key file.
    KeyReadError {
        path: String,
        source: std::io::Error,
    },
    /// Invalid certificate format.
    InvalidCertificate { message: String },
    /// TLS configuration error.
    TlsConfigError { message: String },
    /// Failed to decrypt private key.
    KeyDecryptionError { message: String },
}

impl std::fmt::Display for TransportCredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CertificateReadError { path, source } => {
                write!(f, "Failed to read certificate file '{}': {}", path, source)
            }
            Self::KeyReadError { path, source } => {
                write!(f, "Failed to read key file '{}': {}", path, source)
            }
            Self::InvalidCertificate { message } => {
                write!(f, "Invalid certificate: {}", message)
            }
            Self::TlsConfigError { message } => {
                write!(f, "TLS configuration error: {}", message)
            }
            Self::KeyDecryptionError { message } => {
                write!(f, "Failed to decrypt private key: {}", message)
            }
        }
    }
}

impl std::error::Error for TransportCredentialsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CertificateReadError { source, .. } => Some(source),
            Self::KeyReadError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// TLS configuration mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsMode {
    /// No TLS (insecure connection).
    Insecure,
    /// Server-side TLS only (server certificate verification).
    ServerTls,
    /// Mutual TLS (both server and client certificates).
    MutualTls,
}

/// Configuration for gRPC transport credentials.
#[derive(Debug, Clone)]
pub struct TransportCredentialsConfig {
    /// TLS mode.
    pub mode: TlsMode,
    /// Server CA certificate PEM data.
    pub server_ca_cert: Option<Vec<u8>>,
    /// Client certificate PEM data.
    pub client_cert: Option<Vec<u8>>,
    /// Client key PEM data.
    pub client_key: Option<Vec<u8>>,
}

impl Default for TransportCredentialsConfig {
    fn default() -> Self {
        Self {
            mode: TlsMode::Insecure,
            server_ca_cert: None,
            client_cert: None,
            client_key: None,
        }
    }
}

/// Returns a gRPC transport credentials configuration based on environment variables.
///
/// The function checks the following environment variables:
/// - `SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE`: If set, enables server-side TLS
/// - `SENZING_TOOLS_CLIENT_CERTIFICATE_FILE` and `SENZING_TOOLS_CLIENT_KEY_FILE`:
///   If both are set (in addition to the server CA), enables mutual TLS
///
/// # Returns
///
/// * `Ok(TransportCredentialsConfig)` - The transport credentials configuration
/// * `Err(TransportCredentialsError)` - If there was an error reading certificate files
///
/// # Example
///
/// ```ignore
/// use senzing_sdk_rust_grpc::helper::{get_grpc_transport_credentials_config, TlsMode};
///
/// let config = get_grpc_transport_credentials_config()?;
/// match config.mode {
///     TlsMode::Insecure => println!("Using insecure connection"),
///     TlsMode::ServerTls => println!("Using server-side TLS"),
///     TlsMode::MutualTls => println!("Using mutual TLS"),
/// }
/// ```
pub fn get_grpc_transport_credentials_config()
-> Result<TransportCredentialsConfig, TransportCredentialsError> {
    let mut config = TransportCredentialsConfig::default();

    // Check for server CA certificate (enables TLS)
    if let Ok(server_ca_path) = env::var(ENV_SERVER_CA_CERTIFICATE_FILE) {
        let server_ca_cert = read_certificate_file(&server_ca_path)?;
        config.server_ca_cert = Some(server_ca_cert);
        config.mode = TlsMode::ServerTls;

        // Check for client certificate and key (enables mTLS)
        let client_cert_path = env::var(ENV_CLIENT_CERTIFICATE_FILE).ok();
        let client_key_path = env::var(ENV_CLIENT_KEY_FILE).ok();

        if let (Some(cert_path), Some(key_path)) = (client_cert_path, client_key_path) {
            let client_cert = read_certificate_file(&cert_path)?;
            let client_key = read_key_file(&key_path)?;

            config.client_cert = Some(client_cert);
            config.client_key = Some(client_key);
            config.mode = TlsMode::MutualTls;
        }
    }

    Ok(config)
}

/// Creates a tonic Channel endpoint with the appropriate TLS configuration.
///
/// This function configures TLS based on environment variables and returns
/// an endpoint that can be used to connect to a gRPC server.
///
/// # Arguments
///
/// * `grpc_url` - The gRPC server URL (e.g., "http://localhost:8261" or "https://localhost:8261")
///
/// # Returns
///
/// * `Ok(Channel)` - A configured gRPC channel
/// * `Err(Box<dyn Error>)` - If there was an error configuring the channel
///
/// # Example
///
/// ```ignore
/// use senzing_sdk_rust_grpc::helper::create_grpc_channel;
///
/// let channel = create_grpc_channel("https://localhost:8261").await?;
/// ```
pub async fn create_grpc_channel(grpc_url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let config = get_grpc_transport_credentials_config()?;

    // Adjust URL scheme based on TLS mode
    let adjusted_url = match config.mode {
        TlsMode::Insecure => {
            // Ensure http:// scheme for insecure connections
            if grpc_url.starts_with("https://") {
                grpc_url.replacen("https://", "http://", 1)
            } else if grpc_url.starts_with("http://") {
                grpc_url.to_string()
            } else {
                // No scheme present, add http://
                format!("http://{}", grpc_url)
            }
        }
        TlsMode::ServerTls | TlsMode::MutualTls => {
            // Ensure https:// scheme for TLS connections
            if grpc_url.starts_with("http://") {
                grpc_url.replacen("http://", "https://", 1)
            } else if grpc_url.starts_with("https://") {
                grpc_url.to_string()
            } else {
                // No scheme present, add https://
                format!("https://{}", grpc_url)
            }
        }
    };

    let mut endpoint = Channel::from_shared(adjusted_url)?;

    match config.mode {
        TlsMode::Insecure => {
            // No TLS configuration needed
            println!(">>>>>>>>>>!! Insecure");
        }
        TlsMode::ServerTls => {
            println!(">>>>>>>>>>!! ServerTls");
            if let Some(ca_cert) = config.server_ca_cert {
                let ca = Certificate::from_pem(ca_cert);
                let tls_config = ClientTlsConfig::new().ca_certificate(ca);
                endpoint = endpoint.tls_config(tls_config)?;
            }
        }
        TlsMode::MutualTls => {
            println!(">>>>>>>>>>!! MutualTls");
            if let (Some(ca_cert), Some(client_cert), Some(client_key)) =
                (config.server_ca_cert, config.client_cert, config.client_key)
            {
                let ca = Certificate::from_pem(ca_cert);
                let identity = Identity::from_pem(client_cert, client_key);
                let tls_config = ClientTlsConfig::new().ca_certificate(ca).identity(identity);
                endpoint = endpoint.tls_config(tls_config)?;
            }
        }
    }

    Ok(endpoint.connect().await?)
}

/// Creates a tonic Channel endpoint with the appropriate TLS configuration (blocking version).
///
/// This is a synchronous wrapper around `create_grpc_channel` for use in non-async contexts.
///
/// # Arguments
///
/// * `grpc_url` - The gRPC server URL
/// * `runtime` - A tokio runtime to execute the async operation
///
/// # Returns
///
/// * `Ok(Channel)` - A configured gRPC channel
/// * `Err(Box<dyn Error>)` - If there was an error configuring the channel
pub fn create_grpc_channel_blocking(
    grpc_url: &str,
    runtime: &tokio::runtime::Runtime,
) -> Result<Channel, Box<dyn std::error::Error>> {
    runtime.block_on(create_grpc_channel(grpc_url))
}

/// Reads a certificate file from the filesystem.
fn read_certificate_file(path: &str) -> Result<Vec<u8>, TransportCredentialsError> {
    let clean_path = Path::new(path);
    fs::read(clean_path).map_err(|e| TransportCredentialsError::CertificateReadError {
        path: path.to_string(),
        source: e,
    })
}

/// Reads a key file from the filesystem.
/// If the key is encrypted (PKCS#8 encrypted format), it will be decrypted
/// using the passphrase from the `SENZING_TOOLS_CLIENT_KEY_PASSPHRASE` environment variable.
fn read_key_file(path: &str) -> Result<Vec<u8>, TransportCredentialsError> {
    let clean_path = Path::new(path);
    let key_data =
        fs::read(clean_path).map_err(|e| TransportCredentialsError::KeyReadError {
            path: path.to_string(),
            source: e,
        })?;

    // Check if the key is encrypted
    let key_str = String::from_utf8_lossy(&key_data);
    if key_str.contains("ENCRYPTED PRIVATE KEY") {
        // Key is encrypted, need to decrypt it
        let passphrase = env::var(ENV_CLIENT_KEY_PASSPHRASE).map_err(|_| {
            TransportCredentialsError::KeyDecryptionError {
                message: format!(
                    "Encrypted private key requires passphrase. Set {} environment variable.",
                    ENV_CLIENT_KEY_PASSPHRASE
                ),
            }
        })?;

        // Decode PEM to DER bytes
        let (label, der_bytes) =
            pkcs8::der::pem::decode_vec(key_str.as_bytes()).map_err(|e| {
                TransportCredentialsError::KeyDecryptionError {
                    message: format!("Failed to decode PEM: {}", e),
                }
            })?;

        // Verify it's an encrypted private key
        if label != EncryptedPrivateKeyInfo::PEM_LABEL {
            return Err(TransportCredentialsError::KeyDecryptionError {
                message: format!("Expected encrypted private key, got: {}", label),
            });
        }

        // Parse the DER-encoded encrypted key
        let encrypted_key = EncryptedPrivateKeyInfo::from_der(&der_bytes).map_err(|e| {
            TransportCredentialsError::KeyDecryptionError {
                message: format!("Failed to parse encrypted key: {}", e),
            }
        })?;

        let decrypted_key: SecretDocument =
            encrypted_key.decrypt(passphrase.as_bytes()).map_err(|e| {
                TransportCredentialsError::KeyDecryptionError {
                    message: format!("Failed to decrypt private key: {}", e),
                }
            })?;

        // Convert back to PEM format
        let pem_string = decrypted_key
            .to_pem("PRIVATE KEY", LineEnding::LF)
            .map_err(|e| TransportCredentialsError::KeyDecryptionError {
                message: format!("Failed to encode decrypted key as PEM: {}", e),
            })?;

        Ok(pem_string.as_bytes().to_vec())
    } else {
        // Key is not encrypted, return as-is
        Ok(key_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;

    // #[test]
    // fn test_tls_mode_default_is_insecure() {
    //     // Ensure env vars are not set for this test
    //     // SAFETY: These are test-only env var manipulations
    //     unsafe {
    //         env::remove_var(ENV_SERVER_CA_CERTIFICATE_FILE);
    //         env::remove_var(ENV_CLIENT_CERTIFICATE_FILE);
    //         env::remove_var(ENV_CLIENT_KEY_FILE);
    //     }

    //     let config = get_grpc_transport_credentials_config().unwrap();
    //     assert_eq!(config.mode, TlsMode::Insecure);
    //     assert!(config.server_ca_cert.is_none());
    //     assert!(config.client_cert.is_none());
    //     assert!(config.client_key.is_none());
    // }

    #[test]
    fn test_transport_credentials_error_display() {
        let error = TransportCredentialsError::InvalidCertificate {
            message: "test error".to_string(),
        };
        assert!(error.to_string().contains("test error"));
    }

    #[test]
    fn test_transport_credentials_config_default() {
        let config = TransportCredentialsConfig::default();
        assert_eq!(config.mode, TlsMode::Insecure);
    }
}
