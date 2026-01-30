//! Macros for test utilities and common patterns.

/// Macro to get the short function name by stripping the first crate prefix
#[macro_export]
macro_rules! short_function_name {
    () => {
        stdext::function_name!()
            .split_once("::")
            .map(|(_, rest)| rest)
            .unwrap_or(stdext::function_name!())
    };
}
