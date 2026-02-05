#[cfg(test)]
mod tests;

pub mod errortypes;

use core::error;
use serde_json::Value;
use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

pub trait SzErrorTrait: Debug + Display + Error + Any {
    fn is(&self, szerror: SzError) -> bool;

    fn error_type(&self) -> SzError;

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SzError {
    SzBadInputError,
    SzConfigurationError,
    SzDatabaseConnectionLostError,
    SzDatabaseError,
    SzDatabaseTransientError,
    #[default]
    SzError,
    SzGeneralError,
    SzLicenseError,
    SzNotFoundError,
    SzNotInitializedError,
    SzReplaceConflictError,
    SzRetryableError,
    SzRetryTimeoutExceededError,
    SzSdkError,
    SzUnhandledError,
    SzUnknownDataSourceError,
    SzUnrecoverableError,
    DebugError,
}

// ----------------------------------------------------------------------------
// SenzingError
// ----------------------------------------------------------------------------

// For explanation of the following technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

#[derive(Default, Debug)]
#[non_exhaustive]
pub struct SenzingError<State = SzError> {
    message: String,
    error_type: SzError,
    error_hierarchy: Vec<SzError>,
    state: std::marker::PhantomData<State>,
}

// impl SenzingError<SzBadInputError> {}

// impl SenzingError<SzConfigurationError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzGeneralError | SzError::SzConfigurationError
//         )
//     }
// }

// impl SenzingError<SzDatabaseConnectionLostError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzRetryableError | SzError::SzDatabaseConnectionLostError
//         )
//     }
// }

impl SenzingError<SzDatabaseError> {
    pub fn mjd_was_here(&self) {}

    // pub fn is(&self, szerror: SzError) -> bool {
    //     matches!(
    //         szerror,
    //         SzError::SzError | SzError::SzUnrecoverableError | SzError::SzDatabaseError
    //     )
    // }
}

// impl SenzingError<SzDatabaseTransientError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzRetryableError | SzError::SzDatabaseTransientError
//         )
//     }
// }

// impl SenzingError<SzError> {}
// impl SenzingError<SzGeneralError> {}

// impl SenzingError<SzLicenseError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzUnrecoverableError | SzError::SzLicenseError
//         )
//     }
// }
// impl SenzingError<SzNotFoundError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzBadInputError | SzError::SzNotFoundError
//         )
//     }
// }
// impl SenzingError<SzNotInitializedError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzUnrecoverableError | SzError::SzNotInitializedError
//         )
//     }
// }
// impl SenzingError<SzReplaceConflictError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzGeneralError | SzError::SzReplaceConflictError
//         )
//     }
// }

// // impl SenzingError<SzRetryableError> {}

// impl SenzingError<SzRetryTimeoutExceededError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzRetryableError | SzError::SzRetryTimeoutExceededError
//         )
//     }
// }

// impl SenzingError<SzSdkError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzGeneralError | SzError::SzSdkError
//         )
//     }
// }

// impl SenzingError<SzUnknownDataSourceError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzBadInputError | SzError::SzUnknownDataSourceError
//         )
//     }
// }

// impl SenzingError<SzUnhandledError> {
//     pub fn is(&self, szerror: SzError) -> bool {
//         matches!(
//             szerror,
//             SzError::SzError | SzError::SzUnrecoverableError | SzError::SzUnhandledError
//         )
//     }
// }

// impl SenzingError<SzUnrecoverableError> {}

impl<State: 'static> SenzingError<State> {
    pub fn message(self) -> String {
        self.message
    }

    pub fn error_type(&self) -> SzError {
        self.error_type
    }

    /// Attempts to downcast this error to a specific SenzingError type.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the downcast succeeds
    /// * `None` if the downcast fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(general_error) = sz_error.downcast::<SenzingError<SzGeneralError>>() {
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

impl<State: Debug + 'static> Error for SenzingError<State> {}

impl<State> Display for SenzingError<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SenzingError: {}", self.message)
    }
}

impl<State: Debug + 'static> SzErrorTrait for SenzingError<State> {
    fn error_type(&self) -> SzError {
        self.error_type
    }

    fn is(&self, szerror: SzError) -> bool {
        let result = self.error_hierarchy.contains(&szerror);
        println!(
            "    >>>>>> is: {:?} in {:?} = {}",
            szerror, self.error_hierarchy, result
        );
        result
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ----------------------------------------------------------------------------
// Constructors
// ----------------------------------------------------------------------------

impl SenzingError {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(message: String) -> Box<dyn SzErrorTrait> {
        let error_type_x = extract_error_type(&message);
        if let Some(error_type) = error_type_x {
            match error_type {
                SzError::SzBadInputError => {
                    println!("    >>>>>> Creating SzBadInputError");
                    Box::new(SenzingError::<SzBadInputError> {
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::SzBadInputError, SzError::SzError],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzConfigurationError => {
                    println!("    >>>>>> Creating SzConfigurationError");
                    Box::new(SenzingError::<SzConfigurationError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzConfigurationError,
                            SzError::SzGeneralError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzDatabaseConnectionLostError => {
                    println!("    >>>>>> Creating SzDatabaseConnectionLostError");
                    Box::new(SenzingError::<SzDatabaseConnectionLostError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzDatabaseConnectionLostError,
                            SzError::SzRetryableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzDatabaseError => {
                    println!("    >>>>>> Creating SzDatabaseError");
                    Box::new(SenzingError::<SzDatabaseError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzDatabaseError,
                            SzError::SzUnrecoverableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzDatabaseTransientError => {
                    println!("    >>>>>> Creating SzDatabaseTransientError");
                    Box::new(SenzingError::<SzDatabaseTransientError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzDatabaseTransientError,
                            SzError::SzRetryableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzGeneralError => {
                    println!("    >>>>>> Creating SzGeneralError");
                    Box::new(SenzingError::<SzGeneralError> {
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::SzGeneralError, SzError::SzError],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzLicenseError => {
                    println!("    >>>>>> Creating SzLicenseError");
                    Box::new(SenzingError::<SzLicenseError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzLicenseError,
                            SzError::SzUnrecoverableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzNotFoundError => {
                    println!("    >>>>>> Creating SzNotFoundError");
                    Box::new(SenzingError::<SzNotFoundError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzNotFoundError,
                            SzError::SzBadInputError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzNotInitializedError => {
                    println!("    >>>>>> Creating SzNotInitializedError");
                    Box::new(SenzingError::<SzNotInitializedError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzNotInitializedError,
                            SzError::SzUnrecoverableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzReplaceConflictError => {
                    println!("    >>>>>> Creating SzReplaceConflictError");
                    Box::new(SenzingError::<SzReplaceConflictError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzReplaceConflictError,
                            SzError::SzGeneralError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzRetryableError => {
                    println!("    >>>>>> Creating SzRetryableError");
                    Box::new(SenzingError::<SzRetryableError> {
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::SzRetryableError, SzError::SzError],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzRetryTimeoutExceededError => {
                    println!("    >>>>>> Creating SzRetryTimeoutExceededError");
                    Box::new(SenzingError::<SzRetryTimeoutExceededError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzRetryTimeoutExceededError,
                            SzError::SzRetryableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzSdkError => {
                    println!("    >>>>>> Creating SzSdkError");
                    Box::new(SenzingError::<SzSdkError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzSdkError,
                            SzError::SzGeneralError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzUnhandledError => {
                    println!("    >>>>>> Creating SzUnhandledError");
                    Box::new(SenzingError::<SzUnhandledError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzUnhandledError,
                            SzError::SzUnrecoverableError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzUnknownDataSourceError => {
                    println!("    >>>>>> Creating SzUnknownDataSourceError");
                    Box::new(SenzingError::<SzUnknownDataSourceError> {
                        message,
                        error_type,
                        error_hierarchy: vec![
                            SzError::SzUnknownDataSourceError,
                            SzError::SzBadInputError,
                            SzError::SzError,
                        ],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzUnrecoverableError => {
                    println!("    >>>>>> Creating SzUnrecoverableError");
                    Box::new(SenzingError::<SzUnrecoverableError> {
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::SzUnrecoverableError, SzError::SzError],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::SzError => {
                    println!("    >>>>>> Creating SzError");
                    Box::new(SenzingError::<SzError> {
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::SzError],
                        state: std::marker::PhantomData,
                    })
                }
                SzError::DebugError => {
                    println!("    >>>>>> Creating DebugError");
                    Box::new(SenzingError::<SzError> {
                        // FIXME:
                        message,
                        error_type,
                        error_hierarchy: vec![SzError::DebugError],
                        state: std::marker::PhantomData,
                    })
                }
            }
        } else {
            // No error type could be extracted, return generic SzError variant
            println!(
                "    >>>>>> Could not create typed SenzingError. error-type: {:?}",
                error_type_x
            );
            Box::new(SenzingError::<SzError> {
                message,
                error_type: SzError::SzError,
                error_hierarchy: vec![SzError::SzError, SzError::DebugError],
                state: std::marker::PhantomData,
            })
        }
    }
}

// ----------------------------------------------------------------------------
// Private functions
// ----------------------------------------------------------------------------

fn extract_error_type(message: &str) -> Option<SzError> {
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

fn get_error_type_for_error_id(error_id: i32) -> Option<SzError> {
    errortypes::SZ_ERROR_TYPES.get(&error_id).copied()
}
