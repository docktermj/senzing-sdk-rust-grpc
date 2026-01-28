#[cfg(test)]
use crate::error::SzError;

#[derive(Debug, Default, PartialEq)]

pub struct TestCase {
    pub message: String,
    pub id: Option<String>,
    pub reason: Option<String>,
    pub function: Option<String>,
    pub error: Option<String>,
    pub error_type: Option<SzError>,
    pub error_id: Option<i32>,
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
            message: r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string(),
            reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzConfigurationError),
            error_id: Some(60),
            ..Default::default()
        },
    ]
}

mod test {
    use super::get_testcases;
    use crate::error::SzError;
    use crate::error::{build_senzing_error, extract_reason_from_json};
    use crate::senzing_error_type1;
    use crate::senzing_error_type2;
    use crate::senzing_error_type3;
    // use crate::senzing_error_type4;
    use serde_json::Value;

    // ------------------------------------------------------------------------
    // Test prototypes of error type
    // ------------------------------------------------------------------------

    #[test]
    fn test_match_senzing_error_type1() {
        let target = SzError::SzNotFoundError;

        match target {
            senzing_error_type1!(SzError::SzBadInputError) => {
                println!("\n>>>>>>match: Is SzBadInputError")
            }
            senzing_error_type1!(SzError::SzGeneralError) => {
                println!("\n>>>>>>match: Is SzGeneralError")
            }
            senzing_error_type1!(SzError::SzRetryableError) => {
                println!("\n>>>>>>match: Is SzRecoverableError")
            }
            senzing_error_type1!(SzError::SzUnrecoverableError) => {
                println!("\n>>>>>>match: Is SzUnrecoverableError")
            }
            senzing_error_type1!(SzError::SzError) => {
                println!("\n>>>>>>match: Is SzError")
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type2() {
        let target = SzError::SzNotFoundError;

        if senzing_error_type2!(SzError::SzBadInputError, target) {
            println!("\n>>>>>>match: Is SzBadInputError")
        } else if senzing_error_type2!(SzError::SzGeneralError, target) {
            println!("\n>>>>>>match: Is SzGeneralError")
        } else if senzing_error_type2!(SzError::SzRetryableError, target) {
            println!("\n>>>>>>match: Is SzRecoverableError")
        } else if senzing_error_type2!(SzError::SzUnrecoverableError, target) {
            println!("\n>>>>>>match: Is SzUnrecoverableError")
        } else if senzing_error_type2!(SzError::SzError, target) {
            println!("\n>>>>>>match: Is SzError")
        }
    }

    #[test]
    fn test_match_senzing_error_type3() {
        let target = SzError::SzNotFoundError;

        match target {
            x if senzing_error_type3!(SzError::SzBadInputError).contains(&x) => {
                println!("\n>>>>>>match: Is SzBadInputError")
            }
            x if senzing_error_type3!(SzError::SzGeneralError).contains(&x) => {
                println!("\n>>>>>>match: Is SzGeneralError")
            }
            x if senzing_error_type3!(SzError::SzRetryableError).contains(&x) => {
                println!("\n>>>>>>match: Is SzRecoverableError")
            }
            x if senzing_error_type3!(SzError::SzUnrecoverableError).contains(&x) => {
                println!("\n>>>>>>match: Is SzUnrecoverableError")
            }
            _ => {
                println!("\n>>>>>>match: Is SzError")
            }
        }
    }

    // #[test]
    // fn test_match_senzing_error_type4() {
    //     let target = SzError::SzNotFoundError;

    //     match target {
    //         senzing_error_type4!(SzError::SzBadInputError) => {
    //             println!("\n>>>>>>match: Is SzBadInputError")
    //         }
    //         senzing_error_type4!(SzError::SzGeneralError) => {
    //             println!("\n>>>>>>match: Is SzGeneralError")
    //         }
    //         senzing_error_type4!(SzError::SzRetryableError) => {
    //             println!("\n>>>>>>match: Is SzRecoverableError")
    //         }
    //         senzing_error_type4!(SzError::SzUnrecoverableError) => {
    //             println!("\n>>>>>>match: Is SzUnrecoverableError")
    //         }
    //         senzing_error_type4!(SzError::SzError) => {
    //             println!("\n>>>>>>match: Is SzError")
    //         }
    //     }
    // }

    // ------------------------------------------------------------------------
    // Test...
    // ------------------------------------------------------------------------

    #[test]
    fn test_senzing_reasons() {
        let testcases = get_testcases();
        for testcase in testcases {
            let senzing_error = build_senzing_error(&testcase.message);
            if let Some(reason) = testcase.reason {
                assert_eq!(reason, senzing_error.reason())
            }
        }
    }

    #[test]
    fn test_senzing_error_types() {
        let testcases = get_testcases();
        for testcase in testcases {
            let senzing_error = build_senzing_error(&testcase.message);
            if let Some(error_type) = testcase.error_type {
                assert_eq!(error_type, senzing_error.error_type().unwrap())
            }
        }
    }

    // #[test]
    // fn test_senzing_error_types() {
    //     let testcases = get_testcases();
    //     for testcase in testcases {
    //         let senzing_error = build_senzing_error(&testcase.message);
    //         if let Some(errortype) = testcase.errortype {
    //             assert_eq!(errortype, senzing_error.error_type())
    //         }
    //     }
    // }

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
