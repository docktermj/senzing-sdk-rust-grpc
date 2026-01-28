#[cfg(test)]
#[derive(Debug, Default, PartialEq)]
pub struct TestCase {
    pub message: String,
    pub id: Option<String>,
    pub reason: Option<String>,
    pub function: Option<String>,
    pub error: Option<String>,
}

pub fn get_testcases() -> Vec<TestCase> {
    vec![
        TestCase {
            message: "bob".to_string(),
            ..Default::default()
        },
        TestCase {
            message: "mary".to_string(),
            ..Default::default()
        },
        TestCase {
            message: r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0057|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string(),
            ..Default::default()
        },
    ]
}

mod test {
    use super::get_testcases;
    use crate::error::SzError;
    use crate::error::{build_senzing_error, extract_reason_from_json};
    use crate::senzing_error_type;
    use serde_json::Value;

    #[test]
    fn test_match_senzing_error_type() {
        let target = SzError::SzNotFoundError;

        match target {
            senzing_error_type!(SzError::SzBadInputError) => {
                println!("\n>>>>>>match: Is SzBadInputError")
            }
            senzing_error_type!(SzError::SzGeneralError) => {
                println!("\n>>>>>>match: Is SzGeneralError")
            }
            senzing_error_type!(SzError::SzRetryableError) => {
                println!("\n>>>>>>match: Is SzRecoverableError")
            }
            senzing_error_type!(SzError::SzUnrecoverableError) => {
                println!("\n>>>>>>match: Is SzUnrecoverableError")
            }
            senzing_error_type!(SzError::SzError) => {
                println!("\n>>>>>>match: Is SzError")
            }
        }
    }

    #[test]
    fn test_match_szerror() {
        let target = SzError::SzNotFoundError;

        match target {
            senzing_error_type!(SzError::SzError) => {
                println!("\n>>>>>>match: Is SzError")
            }
        }
    }

    #[test]
    fn test_loop() {
        let test_cases = get_testcases();
        for test_case in test_cases {
            println!(
                "message: {}, reason: {:?}",
                test_case.message, test_case.reason
            );
        }
    }

    #[test]
    fn test_reason_from_direct_json() {
        let error =
            build_senzing_error(r#"rpc error: code = Unknown desc = {"reason": "SZSDK00010001"}"#);
        assert_eq!(error.reason(), "SZSDK00010001");
    }

    #[test]
    fn test_reason_from_nested_json() {
        let error = build_senzing_error(
            r#"rpc error: code = Unknown desc = {"error": {"reason": "SZSDK00020002"}}"#,
        );
        assert_eq!(error.reason(), "SZSDK00020002");
    }

    #[test]
    fn test_reason_no_json() {
        let error = build_senzing_error("rpc error: code = Unknown desc = plain text error");
        assert_eq!(error.reason(), "");
    }

    #[test]
    fn test_reason_json_without_reason_field() {
        let error = build_senzing_error(
            r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#,
        );
        assert_eq!(error.reason(), "");
    }

    #[test]
    fn test_extract_reason_from_json_direct() {
        let json: Value = serde_json::json!({
            "reason": "SZSDK00010001"
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, Some("SZSDK00010001".to_string()));
    }

    #[test]
    fn test_extract_reason_from_json_nested() {
        let json: Value = serde_json::json!({
            "error": {
                "reason": "SZSDK00020002"
            }
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, Some("SZSDK00020002".to_string()));
    }

    #[test]
    fn test_extract_reason_from_json_deeply_nested() {
        let json: Value = serde_json::json!({
            "error": {
                "error": {
                    "reason": "SZSDK00030003"
                }
            }
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, Some("SZSDK00030003".to_string()));
    }

    #[test]
    fn test_extract_reason_from_json_no_reason() {
        let json: Value = serde_json::json!({
            "message": "error occurred"
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, None);
    }

    #[test]
    fn test_reason_from_grpc_error_with_escaped_json() {
        // This tests the actual format we see from gRPC errors with deeply nested reason
        let error_msg = r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": {\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0057|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#;
        let error = build_senzing_error(error_msg);
        assert_eq!(error.reason(), "SENZ0057|Unknown feature ID value '1'");
    }
}
