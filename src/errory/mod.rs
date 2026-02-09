mod error_mappings_generated;

/// Senzing SDK component for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SzComponent {
    Engine,
    Config,
    ConfigMgr,
    Diagnostic,
    Product,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    // Base categories
    BadInput,
    Retryable,
    Unrecoverable,

    // Specific types under BadInput
    NotFound,
    UnknownDataSource,

    // Specific types under Retryable
    DatabaseConnectionLost,
    DatabaseTransient,
    RetryTimeoutExceeded,

    // Specific types under Unrecoverable
    Database,
    License,
    NotInitialized,
    Unhandled,

    // Standalone types
    Configuration,
    ReplaceConflict,
    EnvironmentDestroyed,
    Unknown,
}

#[derive(Debug)]
pub struct ErrorContext {
    /// Human-readable error message
    pub message: String,
    /// Optional Senzing native error code
    pub code: Option<i64>,
    /// Optional SDK component that generated the error
    pub component: Option<SzComponent>,
    /// Optional underlying cause of this error
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ErrorContext {
    /// Creates a new ErrorContext with just a message
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            code: None,
            component: None,
            source: None,
        }
    }

    /// Creates an ErrorContext with message, code, and component
    pub fn with_code<S: Into<String>>(message: S, code: i64, component: SzComponent) -> Self {
        Self {
            message: message.into(),
            code: Some(code),
            component: Some(component),
            source: None,
        }
    }

    /// Adds a source error to this context
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(code) = self.code {
            write!(f, " (code: {})", code)?;
        }
        Ok(())
    }
}

pub type SzResult<T> = Result<T, SzError>;

pub trait SzResultExt<T> {
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>;

    fn map_retryable<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>;

    fn filter_retryable(self) -> Result<Option<T>, SzError>;

    /// Returns true if the result is an error and that error is retryable
    fn is_retryable_error(&self) -> bool;

    /// Returns true if the result is an error and that error is unrecoverable
    fn is_unrecoverable_error(&self) -> bool;

    /// Returns true if the result is an error and that error is bad input
    fn is_bad_input_error(&self) -> bool;
}

impl<T> SzResultExt<T> for SzResult<T> {
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(e) if e.is_retryable() => f(e),
            Err(e) => Err(e),
        }
    }

    fn map_retryable<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>,
    {
        self.or_retry(f)
    }

    fn filter_retryable(self) -> Result<Option<T>, SzError> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(e) if e.is_retryable() => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn is_retryable_error(&self) -> bool {
        matches!(self, Err(e) if e.is_retryable())
    }

    fn is_unrecoverable_error(&self) -> bool {
        matches!(self, Err(e) if e.is_unrecoverable())
    }

    fn is_bad_input_error(&self) -> bool {
        matches!(self, Err(e) if e.is_bad_input())
    }
}

pub trait SzErrorInspect {
    fn sz_error(&self) -> Option<&SzError>;

    fn is_sz_retryable(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_retryable())
    }

    fn is_sz_unrecoverable(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_unrecoverable())
    }

    fn is_sz_bad_input(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_bad_input())
    }

    fn is_sz(&self, category: ErrorCategory) -> bool {
        self.sz_error().is_some_and(|e| e.is(category))
    }
}

impl<E: std::error::Error + 'static> SzErrorInspect for E {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + Sync + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SzError {
    /// Errors related to invalid input parameters
    BadInput(ErrorContext),

    /// Configuration-related errors
    Configuration(ErrorContext),

    /// Database operation errors
    Database(ErrorContext),

    /// License-related errors
    License(ErrorContext),

    /// Resource not found errors
    NotFound(ErrorContext),

    /// Errors that indicate the operation should be retried
    Retryable(ErrorContext),

    /// Unrecoverable errors that require reinitialization
    Unrecoverable(ErrorContext),

    /// Unknown or unexpected errors
    Unknown(ErrorContext),

    /// System not initialized errors
    NotInitialized(ErrorContext),

    /// Database connection lost errors
    DatabaseConnectionLost(ErrorContext),

    /// Database transient errors (e.g., deadlocks)
    DatabaseTransient(ErrorContext),

    /// Replace conflict errors
    ReplaceConflict(ErrorContext),

    /// Retry timeout exceeded errors
    RetryTimeoutExceeded(ErrorContext),

    /// Unhandled errors
    Unhandled(ErrorContext),

    /// Unknown data source errors
    UnknownDataSource(ErrorContext),
}

// Manual implementation of Display and Error traits
impl std::fmt::Display for SzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadInput(ctx) => write!(f, "Bad input: {}", ctx),
            Self::Configuration(ctx) => write!(f, "Configuration error: {}", ctx),
            Self::Database(ctx) => write!(f, "Database error: {}", ctx),
            Self::License(ctx) => write!(f, "License error: {}", ctx),
            Self::NotFound(ctx) => write!(f, "Not found: {}", ctx),
            Self::Retryable(ctx) => write!(f, "Retryable error: {}", ctx),
            Self::Unrecoverable(ctx) => write!(f, "Unrecoverable error: {}", ctx),
            Self::Unknown(ctx) => write!(f, "Unknown error: {}", ctx),
            Self::NotInitialized(ctx) => write!(f, "Not initialized: {}", ctx),
            Self::DatabaseConnectionLost(ctx) => write!(f, "Database connection lost: {}", ctx),
            Self::DatabaseTransient(ctx) => write!(f, "Database transient error: {}", ctx),
            Self::ReplaceConflict(ctx) => write!(f, "Replace conflict: {}", ctx),
            Self::RetryTimeoutExceeded(ctx) => write!(f, "Retry timeout exceeded: {}", ctx),
            Self::Unhandled(ctx) => write!(f, "Unhandled error: {}", ctx),
            Self::UnknownDataSource(ctx) => write!(f, "Unknown data source: {}", ctx),
            /* SzSdkError */
        }
    }
}

impl std::error::Error for SzError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx) => {
                ctx.source.as_ref().map(|e| &**e as &dyn std::error::Error)
            }
        }
    }
}

impl SzError {
    /// Creates a new BadInput error
    pub fn bad_input<S: Into<String>>(message: S) -> Self {
        Self::BadInput(ErrorContext::new(message))
    }

    /// Creates a new Configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration(ErrorContext::new(message))
    }

    /// Creates a new Database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database(ErrorContext::new(message))
    }

    /// Creates a new License error
    pub fn license<S: Into<String>>(message: S) -> Self {
        Self::License(ErrorContext::new(message))
    }

    /// Creates a new NotFound error
    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Self::NotFound(ErrorContext::new(message))
    }

    /// Creates a new Retryable error
    pub fn retryable<S: Into<String>>(message: S) -> Self {
        Self::Retryable(ErrorContext::new(message))
    }

    /// Creates a new Unrecoverable error
    pub fn unrecoverable<S: Into<String>>(message: S) -> Self {
        Self::Unrecoverable(ErrorContext::new(message))
    }

    /// Creates a new Unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown(ErrorContext::new(message))
    }

    /// Creates a new NotInitialized error
    pub fn not_initialized<S: Into<String>>(message: S) -> Self {
        Self::NotInitialized(ErrorContext::new(message))
    }

    /// Creates a new DatabaseConnectionLost error
    pub fn database_connection_lost<S: Into<String>>(message: S) -> Self {
        Self::DatabaseConnectionLost(ErrorContext::new(message))
    }

    /// Creates a new DatabaseTransient error
    pub fn database_transient<S: Into<String>>(message: S) -> Self {
        Self::DatabaseTransient(ErrorContext::new(message))
    }

    /// Creates a new ReplaceConflict error
    pub fn replace_conflict<S: Into<String>>(message: S) -> Self {
        Self::ReplaceConflict(ErrorContext::new(message))
    }

    /// Creates a new RetryTimeoutExceeded error
    pub fn retry_timeout_exceeded<S: Into<String>>(message: S) -> Self {
        Self::RetryTimeoutExceeded(ErrorContext::new(message))
    }

    /// Creates a new Unhandled error
    pub fn unhandled<S: Into<String>>(message: S) -> Self {
        Self::Unhandled(ErrorContext::new(message))
    }

    /// Creates a new UnknownDataSource error
    pub fn unknown_data_source<S: Into<String>>(message: S) -> Self {
        Self::UnknownDataSource(ErrorContext::new(message))
    }

    /// Creates a new EnvironmentDestroyed error
    pub fn environment_destroyed<S: Into<String>>(message: S) -> Self {
        Self::EnvironmentDestroyed(ErrorContext::new(message))
    }

    // ========================================================================
    // Error Chain Inspection - Static Methods
    // ========================================================================

    pub fn find_in_chain<'a>(err: &'a (dyn std::error::Error + 'static)) -> Option<&'a SzError> {
        // Check the error itself
        if let Some(sz) = err.downcast_ref::<SzError>() {
            return Some(sz);
        }
        // Walk the source chain
        let mut source = err.source();
        while let Some(err) = source {
            if let Some(sz) = err.downcast_ref::<SzError>() {
                return Some(sz);
            }
            source = err.source();
        }
        None
    }

    // ========================================================================
    // Error Inspection - Helper Methods
    // ========================================================================

    pub fn error_code(&self) -> Option<i64> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => ctx.code,
            Self::Json(_) | Self::StringConversion(_) => None,
        }
    }

    pub fn component(&self) -> Option<SzComponent> {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => ctx.component,
            Self::Json(_) | Self::StringConversion(_) => None,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => &ctx.message,
            Self::Json(_) => "JSON error",
            Self::StringConversion(_) => "String conversion error",
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SzError::Retryable(_)
                | SzError::DatabaseConnectionLost(_)
                | SzError::DatabaseTransient(_)
                | SzError::RetryTimeoutExceeded(_)
        )
    }

    pub fn is_unrecoverable(&self) -> bool {
        matches!(
            self,
            SzError::Unrecoverable(_)
                | SzError::Database(_)
                | SzError::License(_)
                | SzError::NotInitialized(_)
                | SzError::Unhandled(_)
        )
    }

    pub fn is_bad_input(&self) -> bool {
        matches!(
            self,
            SzError::BadInput(_) | SzError::NotFound(_) | SzError::UnknownDataSource(_)
        )
    }

    pub fn is_database(&self) -> bool {
        matches!(
            self,
            SzError::Database(_)
                | SzError::DatabaseConnectionLost(_)
                | SzError::DatabaseTransient(_)
        )
    }

    pub fn is_license(&self) -> bool {
        matches!(self, SzError::License(_))
    }

    pub fn is_configuration(&self) -> bool {
        matches!(self, SzError::Configuration(_))
    }

    pub fn is_initialization(&self) -> bool {
        matches!(self, SzError::NotInitialized(_))
    }

    pub fn hierarchy(&self) -> Vec<ErrorCategory> {
        // If we have an error code, use the generated hierarchy
        if let Some(code) = self.error_code() {
            let generated = error_mappings_generated::get_error_hierarchy(code);
            if !generated.is_empty() {
                return generated;
            }
        }

        // Fallback to manual mapping for errors without codes
        match self {
            // BadInput family
            Self::BadInput(_) => vec![ErrorCategory::BadInput],
            Self::NotFound(_) => vec![ErrorCategory::NotFound, ErrorCategory::BadInput],
            Self::UnknownDataSource(_) => {
                vec![ErrorCategory::UnknownDataSource, ErrorCategory::BadInput]
            }

            // Retryable family
            Self::Retryable(_) => vec![ErrorCategory::Retryable],
            Self::DatabaseConnectionLost(_) => {
                vec![
                    ErrorCategory::DatabaseConnectionLost,
                    ErrorCategory::Retryable,
                ]
            }
            Self::DatabaseTransient(_) => {
                vec![ErrorCategory::DatabaseTransient, ErrorCategory::Retryable]
            }
            Self::RetryTimeoutExceeded(_) => {
                vec![
                    ErrorCategory::RetryTimeoutExceeded,
                    ErrorCategory::Retryable,
                ]
            }

            // Unrecoverable family
            Self::Unrecoverable(_) => vec![ErrorCategory::Unrecoverable],
            Self::Database(_) => vec![ErrorCategory::Database, ErrorCategory::Unrecoverable],
            Self::License(_) => vec![ErrorCategory::License, ErrorCategory::Unrecoverable],
            Self::NotInitialized(_) => {
                vec![ErrorCategory::NotInitialized, ErrorCategory::Unrecoverable]
            }
            Self::Unhandled(_) => vec![ErrorCategory::Unhandled, ErrorCategory::Unrecoverable],

            // Standalone types
            Self::Configuration(_) => vec![ErrorCategory::Configuration],
            Self::ReplaceConflict(_) => vec![ErrorCategory::ReplaceConflict],
            Self::EnvironmentDestroyed(_) => vec![ErrorCategory::EnvironmentDestroyed],
            Self::Unknown(_) => vec![ErrorCategory::Unknown],

            // FFI errors (no hierarchy)
            Self::Ffi(_) | Self::Json(_) | Self::StringConversion(_) => vec![],
        }
    }

    pub fn is(&self, category: ErrorCategory) -> bool {
        self.hierarchy().contains(&category)
    }

    // ========================================================================
    // Error Metadata - For Error Reporting Integration
    // ========================================================================

    pub fn category(&self) -> &'static str {
        match self {
            Self::BadInput(_) | Self::NotFound(_) | Self::UnknownDataSource(_) => "bad_input",
            Self::Configuration(_) => "configuration",
            Self::Database(_) => "database",
            Self::DatabaseConnectionLost(_) => "database_connection",
            Self::DatabaseTransient(_) => "database_transient",
            Self::License(_) => "license",
            Self::NotInitialized(_) => "not_initialized",
            Self::Retryable(_) | Self::RetryTimeoutExceeded(_) => "retryable",
            Self::Unrecoverable(_) | Self::Unhandled(_) => "unrecoverable",
            Self::ReplaceConflict(_) => "replace_conflict",
            Self::EnvironmentDestroyed(_) => "environment_destroyed",
            Self::Unknown(_) => "unknown",
            Self::Json(_) => "json",
            Self::StringConversion(_) => "string_conversion",
        }
    }

    pub fn severity(&self) -> &'static str {
        match self {
            Self::License(_) | Self::Unrecoverable(_) | Self::Unhandled(_) => "critical",
            Self::Database(_) | Self::NotInitialized(_) => "high",
            Self::DatabaseConnectionLost(_)
            | Self::DatabaseTransient(_)
            | Self::Configuration(_) => "medium",
            _ => "low",
        }
    }

    // ========================================================================
    // Error Code Mapping - From Native Senzing Errors
    // ========================================================================

    pub fn from_code_with_message(error_code: i64, component: SzComponent) -> Self {
        let error_msg = Self::get_last_exception_message(component, error_code);
        let ctx = ErrorContext::with_code(error_msg, error_code, component);

        // Use generated error mapping (456 error codes from szerrors.json)
        error_mappings_generated::map_error_code(error_code, ctx)
    }

    /// Creates an error from getLastExceptionCode() (legacy method for compatibility)
    pub fn from_code(error_code: i64) -> Self {
        // Default to Engine component for backward compatibility
        Self::from_code_with_message(error_code, SzComponent::Engine)
    }

    /// Creates an Unknown error from a source error
    pub fn from_source(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        let message = source.to_string();
        Self::Unknown(ErrorContext {
            message,
            code: None,
            component: None,
            source: Some(source),
        })
    }

    /// Creates an Unknown error with a custom message and source
    pub fn with_message_and_source<S: Into<String>>(
        message: S,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::Unknown(ErrorContext {
            message: message.into(),
            code: None,
            component: None,
            source: Some(source),
        })
    }
}

// ========================================================================
// ErrorContext Extension Methods
// ========================================================================

impl ErrorContext {
    pub fn chain_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

// ========================================================================
// SzError Extension Methods for Builder Pattern
// ========================================================================

impl SzError {
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        match &mut self {
            Self::BadInput(ctx)
            | Self::Configuration(ctx)
            | Self::Database(ctx)
            | Self::License(ctx)
            | Self::NotFound(ctx)
            | Self::Retryable(ctx)
            | Self::Unrecoverable(ctx)
            | Self::Unknown(ctx)
            | Self::NotInitialized(ctx)
            | Self::DatabaseConnectionLost(ctx)
            | Self::DatabaseTransient(ctx)
            | Self::ReplaceConflict(ctx)
            | Self::RetryTimeoutExceeded(ctx)
            | Self::Unhandled(ctx)
            | Self::UnknownDataSource(ctx)
            | Self::EnvironmentDestroyed(ctx)
            | Self::Ffi(ctx) => {
                ctx.source = Some(Box::new(source));
            }
            // Json and StringConversion already have their source
            Self::Json(_) | Self::StringConversion(_) => {}
        }
        self
    }
}
