#[cfg(test)]
use crate::error::SzError;

#[derive(Debug, Default, PartialEq)]
pub struct TestCase {
    pub name: &'static str,
    pub message: String,
    pub id: Option<String>,
    pub reason: Option<String>,
    pub function: Option<String>,
    pub error: Option<String>,
    pub error_type: Option<SzError>,
    pub error_id: Option<i32>,
    pub should_fail: bool, // Defaults to false.
}

pub fn get_testcases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "Empty message",
            message: "".to_string(),
            ..Default::default()
        },
        TestCase {
            name: "No JSON message",
            message: "No JSON message".to_string(),
            ..Default::default()
        },
        TestCase {
            name: "SzConfigurationError",
            message: r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string(),
            reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzConfigurationError),
            error_id: Some(60),
            ..Default::default()
        },
        TestCase {
            name: "SzBadInputError",
            message: r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string(),
            reason: Some("SENZ3131|Invalid column [BAD] requested for CSV export.".to_string()),
            error_type: Some(SzError::SzBadInputError),
            error_id: Some(3131),
            ..Default::default()
        },
        TestCase {
            name: "NegativeTest",
            message: r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string(),
            reason: Some("SENZ3132|Invalid column [BAD] requested for CSV export.".to_string()), // Wrong SENZnnnn number
            error_type: Some(SzError::SzConfigurationError), // Wrong error_type
            error_id: Some(3132), // Wrong ID
            should_fail: true,
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All None",
            message: r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string(),
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All Some",
            message: r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string(),
            reason: Some("".to_string()),
            error_type: Some(SzError::SzError),
            error_id: Some(0),
            ..Default::default()
        },
    ]
}

pub fn mock_senzing_function(message: String) -> Result<String, Box<dyn std::error::Error>> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        message,
    )))
}

mod test {
    use super::{get_testcases, mock_senzing_function};
    use crate::error::SzError;
    use crate::error::{
        build_senzing_error, build_senzing_error_from_err, extract_reason_from_json,
        is_senzing_error_type,
    };
    use crate::senzing_error_type1;
    use crate::senzing_error_type2;
    use crate::senzing_error_type3;
    // use crate::short_function_name;
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
                println!(">>>>>> match1: Is SzBadInputError")
            }
            senzing_error_type1!(SzError::SzGeneralError) => {
                println!(">>>>>> match1: Is SzGeneralError")
            }
            senzing_error_type1!(SzError::SzRetryableError) => {
                println!(">>>>>> match1: Is SzRecoverableError")
            }
            senzing_error_type1!(SzError::SzUnrecoverableError) => {
                println!(">>>>>> match1: Is SzUnrecoverableError")
            }
            senzing_error_type1!(SzError::SzError) => {
                println!(">>>>>> match1: Is SzError")
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type2() {
        let target = SzError::SzNotFoundError;

        if senzing_error_type2!(SzError::SzBadInputError, target) {
            println!(">>>>>> match2: Is SzBadInputError")
        } else if senzing_error_type2!(SzError::SzGeneralError, target) {
            println!(">>>>>> match2: Is SzGeneralError")
        } else if senzing_error_type2!(SzError::SzRetryableError, target) {
            println!(">>>>>> match2: Is SzRecoverableError")
        } else if senzing_error_type2!(SzError::SzUnrecoverableError, target) {
            println!(">>>>>> match2: Is SzUnrecoverableError")
        } else if senzing_error_type2!(SzError::SzError, target) {
            println!(">>>>>> match2: Is SzError")
        }
    }

    #[test]
    fn test_match_senzing_error_type3() {
        let target = SzError::SzNotFoundError;

        match target {
            x if senzing_error_type3!(SzError::SzBadInputError).contains(&x) => {
                println!(">>>>>> match3: Is SzBadInputError")
            }
            x if senzing_error_type3!(SzError::SzGeneralError).contains(&x) => {
                println!(">>>>>> match3: Is SzGeneralError")
            }
            x if senzing_error_type3!(SzError::SzRetryableError).contains(&x) => {
                println!(">>>>>> match3: Is SzRecoverableError")
            }
            x if senzing_error_type3!(SzError::SzUnrecoverableError).contains(&x) => {
                println!(">>>>>> match3: Is SzUnrecoverableError")
            }
            _ => {
                println!(">>>>>> match3: Is SzError")
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type4() {
        let target = SzError::SzNotFoundError;

        match target {
            x if is_senzing_error_type(x, SzError::SzBadInputError) => {
                println!(">>>>>> match: Is SzBadInputError")
            }
            x if is_senzing_error_type(x, SzError::SzGeneralError) => {
                println!(">>>>>> match: Is SzGeneralError")
            }
            x if is_senzing_error_type(x, SzError::SzRetryableError) => {
                println!(">>>>>> match: Is SzRecoverableError")
            }
            x if is_senzing_error_type(x, SzError::SzUnrecoverableError) => {
                println!(">>>>>> match: Is SzUnrecoverableError")
            }
            x if is_senzing_error_type(x, SzError::SzError) => {
                println!(">>>>>> match: Is SzError")
            }
            _ => {
                println!(">>>>>> match: All other errors")
            }
        }
    }

    // ------------------------------------------------------------------------
    // Test...
    // ------------------------------------------------------------------------

    #[test]
    fn test_senzing_reasons() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(reason) = testcase.reason {
                println!("    expected reason: {}", reason);
                let senzing_error = build_senzing_error(testcase.message);
                if let Some(senzing_reason) = senzing_error.reason() {
                    if testcase.should_fail {
                        println!("    negative testcase");
                        assert_ne!(reason, senzing_reason)
                    } else {
                        assert_eq!(reason, senzing_reason)
                    }
                } else {
                    println!("    error_type() returned None");
                }
            } else {
                println!("    ignored");
            }
        }
    }

    #[test]
    fn test_senzing_error_types() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(error_type) = testcase.error_type {
                println!("    expected error_type: {:?}", error_type);
                let senzing_error = build_senzing_error(&testcase.message);
                if let Some(senzing_error_type) = senzing_error.error_type() {
                    if testcase.should_fail {
                        println!("    negative testcase");
                        assert_ne!(error_type, senzing_error_type)
                    } else {
                        assert_eq!(error_type, senzing_error_type)
                    }
                } else {
                    println!("    error_type() returned None");
                }
            } else {
                println!("    ignored");
            }
        }
    }

    #[test]
    fn test_senzing_error_types2() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(error_type) = testcase.error_type {
                println!("    expected error_type: {:?}", error_type);
                let senzing_result = mock_senzing_function(testcase.message);
                if let Err(e) = senzing_result {
                    let senzing_error = build_senzing_error_from_err(e);
                    if let Some(sz) = senzing_error.error_type() {
                        match sz {
                            senzing_error_type1!(SzError::SzBadInputError) => {
                                println!(">>>>>> match1: Is SzBadInputError")
                            }
                            senzing_error_type1!(SzError::SzGeneralError) => {
                                println!(">>>>>> match1: Is SzGeneralError")
                            }
                            senzing_error_type1!(SzError::SzRetryableError) => {
                                println!(">>>>>> match1: Is SzRecoverableError")
                            }
                            senzing_error_type1!(SzError::SzUnrecoverableError) => {
                                println!(">>>>>> match1: Is SzUnrecoverableError")
                            }
                            senzing_error_type1!(SzError::SzError) => {
                                println!(">>>>>> match1: Is SzError")
                            }
                        }
                    }
                } else {
                    println!("    error_type() returned None");
                }
            } else {
                println!("    ignored");
            }
        }
    }

    #[test]
    fn test_senzing_error_types3() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(error_type) = testcase.error_type {
                println!("    expected error_type: {:?}", error_type);
                let senzing_result = mock_senzing_function(testcase.message);
                match senzing_result {
                    Ok(senzing_string) => {
                        println!("    string: {}", senzing_string);
                    }
                    Err(senzing_error) => {
                        match build_senzing_error_from_err(senzing_error).error_type() {
                            Some(senzing_error_type1!(SzError::SzBadInputError)) => {
                                println!(">>>>>> match1: Is SzBadInputError")
                            }
                            Some(senzing_error_type1!(SzError::SzGeneralError)) => {
                                println!(">>>>>> match1: Is SzGeneralError")
                            }
                            Some(senzing_error_type1!(SzError::SzRetryableError)) => {
                                println!(">>>>>> match1: Is SzRecoverableError")
                            }
                            Some(senzing_error_type1!(SzError::SzUnrecoverableError)) => {
                                println!(">>>>>> match1: Is SzUnrecoverableError")
                            }
                            None => {
                                println!(">>>>>> match1: Is None")
                            }
                            senzing_error_type1!(SzError::SzError) => {
                                println!(">>>>>> match1: Is SzError")
                            }
                        }
                    }
                }
            } else {
                println!("    ignored");
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
        assert_eq!(error.reason(), Some("SZSDK00010001".to_string()));
    }

    #[test]
    fn test_reason_from_nested_json() {
        let error = build_senzing_error(
            r#"rpc error: code = Unknown desc = {"error": {"reason": "SZSDK00020002"}}"#,
        );
        assert_eq!(error.reason(), Some("SZSDK00020002".to_string()));
    }

    #[test]
    fn test_reason_no_json() {
        let error = build_senzing_error("rpc error: code = Unknown desc = plain text error");
        assert_eq!(error.reason(), None);
    }

    #[test]
    fn test_reason_json_without_reason_field() {
        let error = build_senzing_error(
            r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#,
        );
        assert_eq!(error.reason(), None);
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
        assert_eq!(
            error.reason(),
            Some("SENZ0057|Unknown feature ID value '1'".to_string())
        );
    }
}
