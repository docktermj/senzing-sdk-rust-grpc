# Claude Code Conversation - Error Handling Improvements

**Date:** 2026-02-03
**Project:** senzing-sdk-rust-grpc

---

## User Request

> Suggest improvements to src/error that make it easier for the user of the API to handle errors.

---

## Analysis

After exploring the current error handling implementation in `src/error/`, I identified the following files:

- `src/error/mod.rs` - Main error module with `SzError` enum and `SenzingError` struct
- `src/error/errortypes.rs` - Generated mapping from error codes to error types
- `src/error/tests.rs` - Test cases

### Current Implementation

- **SzError enum** - 17 variants representing different error types
- **SenzingError struct** - Wrapper containing original message and extracted JSON
- **as_senzing_error()** - Converts any error to SenzingError
- **Multiple macros** (senzing_error_type1-5) for matching error categories
- **Methods**: `reason()`, `error_type()`, `is_error_type()`

### Issues Identified

1. SDK returns `Result<T, Box<dyn std::error::Error>>` losing type information
2. Users must manually call `as_senzing_error()` to extract error info
3. No `From` implementations for easy conversion
4. Complex macros with known issues (FIXME comments)
5. `is_error_type()` re-parses JSON every time
6. No direct error code access method

---

## Suggested Improvements

### 1. Use a Custom Error Type Instead of `Box<dyn Error>`

**Current:**
```rust
fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>>;
```

**Suggested:**
```rust
fn get_version(&mut self) -> Result<String, SenzingError>;
```

### 2. Add `From` Implementations

```rust
impl From<tonic::Status> for SenzingError {
    fn from(status: tonic::Status) -> Self {
        as_senzing_error(status.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for SenzingError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        as_senzing_error(err)
    }
}
```

### 3. Cache Parsed Error Information

```rust
#[derive(Debug)]
pub struct SenzingError {
    message: String,
    error_type: Option<SzError>,
    error_code: Option<i32>,
    reason: Option<String>,
}

impl SenzingError {
    pub fn reason(&self) -> Option<&str> { self.reason.as_deref() }
    pub fn error_type(&self) -> Option<SzError> { self.error_type }
    pub fn error_code(&self) -> Option<i32> { self.error_code }
}
```

### 4. Add `error_code()` Method

```rust
impl SenzingError {
    /// Returns the Senzing error code (e.g., 33, 1006, 2207)
    pub fn error_code(&self) -> Option<i32> {
        self.error_code
    }
}
```

### 5. Simplify Error Type Matching with Methods

Replace macros with simple methods on `SzError`:

```rust
impl SzError {
    pub fn is_bad_input(&self) -> bool {
        matches!(self,
            SzError::SzBadInputError |
            SzError::SzNotFoundError |
            SzError::SzUnknownDataSourceError
        )
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self,
            SzError::SzRetryableError |
            SzError::SzDatabaseConnectionLostError |
            SzError::SzDatabaseTransientError |
            SzError::SzRetryTimeoutExceededError
        )
    }

    pub fn is_unrecoverable(&self) -> bool {
        matches!(self,
            SzError::SzUnrecoverableError |
            SzError::SzDatabaseError |
            SzError::SzLicenseError |
            SzError::SzNotInitializedError |
            SzError::SzUnhandledError
        )
    }
}
```

And on `SenzingError`:

```rust
impl SenzingError {
    pub fn is_retryable(&self) -> bool {
        self.error_type.map_or(false, |t| t.is_retryable())
    }

    pub fn is_not_found(&self) -> bool {
        self.error_type == Some(SzError::SzNotFoundError)
    }

    pub fn is_senzing_error(&self) -> bool {
        self.error_type.is_some()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
```

### 6. Example User Code Comparison

**Before (current):**
```rust
let result = sz_product.get_version();
match result {
    Ok(version) => println!("{}", version),
    Err(e) => {
        let senzing_error = as_senzing_error(e);
        if let Some(error_type) = senzing_error.error_type() {
            match error_type {
                senzing_error_type1!(SzError::SzRetryableError) => {
                    // retry logic
                }
                _ => {}
            }
        }
    }
}
```

**After (improved):**
```rust
let result = sz_product.get_version();
match result {
    Ok(version) => println!("{}", version),
    Err(e) if e.is_retryable() => {
        // retry logic
    }
    Err(e) if e.is_not_found() => {
        println!("Record not found: {}", e.reason().unwrap_or("unknown"));
    }
    Err(e) => {
        println!("Error {}: {}", e.error_code().unwrap_or(0), e);
    }
}
```

---

## Summary of Recommended Changes

| Change | Benefit |
|--------|---------|
| Return `SenzingError` instead of `Box<dyn Error>` | Type safety, better IDE support |
| Add `From` implementations | Enable `?` operator |
| Cache parsed error data | Performance |
| Add `error_code()` method | Access specific error codes |
| Replace macros with methods | Simpler, safer API |
| Add `is_retryable()`, `is_not_found()`, etc. | Ergonomic error handling |

---
