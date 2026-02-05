#[cfg(test)]
mod tests;

pub mod errortypes;

// Re-export the macro so it's accessible as crate::errorx::extract_senzing_error
pub use crate::extract_senzing_error;

use serde_json::Value;
use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

pub trait SzErrorTrait: Debug + Display + Error + Any {
    fn is(&self, szerror: SzErrorTypes) -> bool;
    fn error_type(&self) -> SzErrorTypes;
    fn message(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct SzBadInputError;

#[derive(Debug)]
pub struct SzConfigurationError;

#[derive(Debug)]
pub struct SzDatabaseConnectionLostError;

#[derive(Debug)]
pub struct SzDatabaseError;

#[derive(Debug)]
pub struct SzDatabaseTransientError;

#[derive(Debug)]
pub struct SzGeneralError;

#[derive(Debug)]
pub struct SzLicenseError;

#[derive(Debug)]
pub struct SzNotFoundError;

#[derive(Debug)]
pub struct SzNotInitializedError;

#[derive(Debug)]
pub struct SzReplaceConflictError;

#[derive(Debug)]
pub struct SzRetryableError;

#[derive(Debug)]
pub struct SzRetryTimeoutExceededError;

#[derive(Debug)]
pub struct SzSdkError;

#[derive(Debug)]
pub struct SzUnhandledError;

#[derive(Debug)]
pub struct SzUnknownDataSourceError;

#[derive(Debug)]
pub struct SzUnrecoverableError;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SzErrorTypes {
    BadInputError,
    ConfigurationError,
    DatabaseConnectionLostError,
    DatabaseError,
    DatabaseTransientError,
    #[default]
    Error,
    GeneralError,
    LicenseError,
    NotFoundError,
    NotInitializedError,
    ReplaceConflictError,
    RetryableError,
    RetryTimeoutExceededError,
    SdkError,
    UnhandledError,
    UnknownDataSourceError,
    UnrecoverableError,
}

// ----------------------------------------------------------------------------
// SzError
// ----------------------------------------------------------------------------

// For explanation of the following technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct SzError<State = SzErrorTypes> {
    message: String,
    error_type: SzErrorTypes,
    error_hierarchy: Vec<SzErrorTypes>,
    state: std::marker::PhantomData<State>,
}

// impl SzError<SzBadInputError> {}
// impl SzError<SzConfigurationError> {}
// impl SzError<SzDatabaseConnectionLostError> {}

impl SzError<SzDatabaseError> {
    pub fn mjd_was_here(&self, suffix: String) {
        println!("    >>>>>> MJD was here -> {}", suffix)
    }
}

// impl SzError<SzDatabaseTransientError> {}
// impl SzError<SzError> {}
// impl SzError<SzGeneralError> {}
// impl SzError<SzLicenseError> {}
// impl SzError<SzNotFoundError> {}
// impl SzError<SzNotInitializedError> {}
// impl SzError<SzReplaceConflictError> {}
// impl SzError<SzRetryableError> {}
// impl SzError<SzRetryTimeoutExceededError> {}
// impl SzError<SzSdkError> {}
// impl SzError<SzUnknownDataSourceError> {}
// impl SzError<SzUnhandledError> {}
// impl SzError<SzUnrecoverableError> {}

impl<State: 'static> SzError<State> {
    // pub fn message(self) -> String {
    //     self.message
    // }

    // pub fn error_type(&self) -> SzErrorTypes {
    //     self.error_type
    // }

    /// Attempts to downcast this error to a specific SzError type.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the downcast succeeds
    /// * `None` if the downcast fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(general_error) = sz_error.downcast::<SzError<SzGeneralError>>() {
    ///     // Handle the general error specifically
    /// }
    /// ```
    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

// ----------------------------------------------------------------------------
// Trait methods
// ----------------------------------------------------------------------------

impl<State: Debug + 'static> Error for SzError<State> {}

impl<State> Display for SzError<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SzError: {}", self.message)
    }
}

impl<State: Debug + 'static> SzErrorTrait for SzError<State> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn error_type(&self) -> SzErrorTypes {
        self.error_type
    }

    fn is(&self, szerror: SzErrorTypes) -> bool {
        self.error_hierarchy.contains(&szerror)
    }

    fn message(&self) -> &str {
        &self.message
    }
}

// ----------------------------------------------------------------------------
// Constructors
// ----------------------------------------------------------------------------

impl SzError {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(message: String) -> Box<dyn SzErrorTrait> {
        let error_type_x = extract_error_type(&message);
        if let Some(error_type) = error_type_x {
            match error_type {
                SzErrorTypes::BadInputError => Box::new(SzError::<SzBadInputError> {
                    message,
                    error_type,
                    error_hierarchy: vec![SzErrorTypes::BadInputError, SzErrorTypes::Error],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::ConfigurationError => Box::new(SzError::<SzConfigurationError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::ConfigurationError,
                        SzErrorTypes::GeneralError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::DatabaseConnectionLostError => {
                    Box::new(SzError::<SzDatabaseConnectionLostError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzErrorTypes::DatabaseConnectionLostError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzErrorTypes::DatabaseError => Box::new(SzError::<SzDatabaseError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::DatabaseError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::DatabaseTransientError => {
                    Box::new(SzError::<SzDatabaseTransientError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzErrorTypes::DatabaseTransientError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzErrorTypes::GeneralError => Box::new(SzError::<SzGeneralError> {
                    message,
                    error_type,
                    error_hierarchy: vec![SzErrorTypes::GeneralError, SzErrorTypes::Error],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::LicenseError => Box::new(SzError::<SzLicenseError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::LicenseError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::NotFoundError => Box::new(SzError::<SzNotFoundError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::NotFoundError,
                        SzErrorTypes::BadInputError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::NotInitializedError => Box::new(SzError::<SzNotInitializedError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::NotInitializedError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::ReplaceConflictError => Box::new(SzError::<SzReplaceConflictError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::ReplaceConflictError,
                        SzErrorTypes::GeneralError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::RetryableError => Box::new(SzError::<SzRetryableError> {
                    message,
                    error_type,
                    error_hierarchy: vec![SzErrorTypes::RetryableError, SzErrorTypes::Error],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::RetryTimeoutExceededError => {
                    Box::new(SzError::<SzRetryTimeoutExceededError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzErrorTypes::RetryTimeoutExceededError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzErrorTypes::SdkError => Box::new(SzError::<SzSdkError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::SdkError,
                        SzErrorTypes::GeneralError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::UnhandledError => Box::new(SzError::<SzUnhandledError> {
                    message,
                    error_type,
                    error_hierarchy: vec![
                        SzErrorTypes::UnhandledError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::UnknownDataSourceError => {
                    Box::new(SzError::<SzUnknownDataSourceError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzErrorTypes::UnknownDataSourceError,
                            SzErrorTypes::BadInputError,
                            SzErrorTypes::Error,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzErrorTypes::UnrecoverableError => Box::new(SzError::<SzUnrecoverableError> {
                    message,
                    error_type,
                    error_hierarchy: vec![SzErrorTypes::UnrecoverableError, SzErrorTypes::Error],
                    state: std::marker::PhantomData,
                }),
                SzErrorTypes::Error => Box::new(SzError::<SzErrorTypes> {
                    message,
                    error_type,
                    error_hierarchy: vec![SzErrorTypes::Error],
                    state: std::marker::PhantomData,
                }),
            }
        } else {
            // No error type could be extracted, return generic SzError variant.
            Box::new(SzError::<SzErrorTypes> {
                message,
                error_type: SzErrorTypes::Error,
                error_hierarchy: vec![SzErrorTypes::Error],
                state: std::marker::PhantomData,
            })
        }
    }
}

// ----------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------

/// Attempts to downcast a `Box<dyn Error>` to `Option<&dyn SzErrorTrait>`.
///
/// This macro tries to downcast the error to all possible `SzError<T>` variants
/// and returns the first successful match as `&dyn SzErrorTrait`.
///
/// # Arguments
///
/// * `$error` - A boxed error (`Box<dyn Error>`)
///
/// # Returns
///
/// * `Option<&dyn SzErrorTrait>` - Some if the error is any variant of `SzError<T>`, None otherwise
///
/// # Example
///
/// ```ignore
/// let boxed_error: Box<dyn Error> = get_some_error();
/// let mary: Option<&dyn SzErrorTrait> = try_downcast_senzing_error!(boxed_error);
/// ```
#[macro_export]
macro_rules! extract_senzing_error {
    ($error:expr) => {
        $error
            .as_ref()
            .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzBadInputError>>()
            .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzConfigurationError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseConnectionLostError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseTransientError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzGeneralError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzLicenseError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzNotFoundError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzNotInitializedError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzReplaceConflictError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzRetryableError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzRetryTimeoutExceededError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzSdkError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnhandledError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnknownDataSourceError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnrecoverableError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                $error
                    .as_ref()
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
    };
}

// ----------------------------------------------------------------------------
// Private functions
// ----------------------------------------------------------------------------

fn extract_error_type(message: &str) -> Option<SzErrorTypes> {
    extract_json_from_message(message)
        .and_then(|json_str| serde_json::from_str::<Value>(&json_str).ok())
        .and_then(|json_value| extract_reason_from_json(&json_value))
        .and_then(|reason| extract_error_id_from_reason(&reason))
        .and_then(get_error_type_for_error_id)
}

fn extract_json_from_message(message: &str) -> Option<String> {
    // Try to find and parse escaped JSON in the "self:" field
    if let Some(self_index) = message.find("self: \"") {
        let json_start = self_index + 7; // Length of "self: \""
        let remaining = &message[json_start..];

        // Find the end of the escaped JSON string (look for closing quote)
        // We need to handle escaped quotes within the JSON
        if let Some(json_end) = find_json_string_end(remaining) {
            let escaped_json = &remaining[..json_end];

            // Unescape the JSON string by replacing escape sequences
            let unescaped_json = escaped_json
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\\\", "\\");

            // Validate it's actually JSON
            if serde_json::from_str::<Value>(&unescaped_json).is_ok() {
                return Some(unescaped_json);
            }
        }
    }

    // Fallback: Look for JSON by finding opening braces
    let brace_index = message.find('{')?;
    let json_str = &message[brace_index..];

    // Validate it's actually JSON and return
    if serde_json::from_str::<Value>(json_str).is_ok() {
        return Some(json_str.to_string());
    }

    None
}

/// Finds the end of an escaped JSON string by looking for the closing quote.
///
/// This function handles escaped quotes within the JSON string.
///
/// # Arguments
///
/// * `s` - The string starting with an escaped JSON value
///
/// # Returns
///
/// * `Some(usize)` - The index of the closing quote
/// * `None` - If no closing quote is found
fn find_json_string_end(s: &str) -> Option<usize> {
    let mut in_escape = false;
    for (i, c) in s.chars().enumerate() {
        if in_escape {
            in_escape = false;
            continue;
        }
        match c {
            '\\' => in_escape = true,
            '"' => return Some(i),
            _ => {}
        }
    }
    None
}

/// Extracts the "reason" field from a JSON value.
///
/// This function recursively searches for the "reason" field in the JSON,
/// checking both the root level and nested "error" objects.
///
/// # Arguments
///
/// * `json_value` - The parsed JSON value to search
///
/// # Returns
///
/// * `Some(String)` if a "reason" field was found
/// * `None` if no "reason" field exists
fn extract_reason_from_json(json_value: &Value) -> Option<String> {
    // Try to get "reason" directly at the root level
    if let Some(reason) = json_value.get("reason").and_then(|v| v.as_str()) {
        return Some(reason.to_string());
    }

    // Try to get "reason" from nested "error" object
    if let Some(error_obj) = json_value.get("error").and_then(|v| v.as_object()) {
        if let Some(reason) = error_obj.get("reason").and_then(|v| v.as_str()) {
            return Some(reason.to_string());
        }
        // Recursively check nested error objects
        if let Some(nested_error) = error_obj.get("error") {
            return extract_reason_from_json(nested_error);
        }
    }

    None
}

/// Extracts the error ID from a reason string.
///
/// The reason string is expected to be in the format "SENZNNNN|description"
/// where NNNN is a numeric error code.
///
/// # Arguments
///
/// * `reason` - The reason string from a Senzing error
///
/// # Returns
///
/// * `Some(i32)` if a valid error ID was found
/// * `None` if the reason string doesn't match the expected format
///
/// # Example
///
/// ```ignore
/// let error_id = extract_error_id_from_reason("SENZ0060|Unknown feature ID");
/// assert_eq!(error_id, Some(60));
/// ```
fn extract_error_id_from_reason(reason: &str) -> Option<i32> {
    // Check if reason starts with "SENZ"
    let after_prefix = reason.strip_prefix("SENZ")?;

    // Find the position of '|' which separates the code from the description
    let pipe_pos = after_prefix.find('|')?;

    // Extract the numeric part and parse it
    let code_str = &after_prefix[..pipe_pos];
    code_str.parse::<i32>().ok()
}

fn get_error_type_for_error_id(error_id: i32) -> Option<SzErrorTypes> {
    errortypes::SZ_ERROR_TYPES.get(&error_id).copied()
}

/// Used to test that types have "normal" traits.
#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin>() {}
