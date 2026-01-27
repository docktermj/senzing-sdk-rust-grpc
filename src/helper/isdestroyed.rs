/// Macro to check if an object is destroyed and return an error if it is.
///
/// # Arguments
/// * `$is_destroyed` - A `Cell<bool>` indicating if the object is destroyed
/// * `$error_msg` - A String error message to return if destroyed
///
/// # Behavior
/// - If `$is_destroyed.get()` evaluates to `true`, returns `Err($error_msg)`
/// - If `$is_destroyed.get()` evaluates to `false`, does nothing and continues execution
///
/// # Example
/// ```ignore
/// use senzing_sdk_rust_grpc::is_destroyed;
/// use std::cell::Cell;
///
/// fn some_method(&self) -> Result<String, Box<dyn std::error::Error>> {
///     is_destroyed!(self.is_destroyed, "Object has been destroyed".to_string());
///     Ok("Success".to_string())
/// }
/// ```
#[macro_export]
macro_rules! is_destroyed {
    ($is_destroyed:expr, $error_msg:expr) => {
        if $is_destroyed.get() {
            return Err($error_msg.into());
        }
    };
}
