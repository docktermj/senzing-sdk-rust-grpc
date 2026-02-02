#[cfg(test)]
use crate::error::SzError;

#[derive(Debug, Default, PartialEq)]
pub struct TestCase {
    pub name: &'static str,
    pub error_id: Option<i32>,
    pub error_message: Option<String>,
    pub error_type_parent: Option<SzError>,
    pub error_type: Option<SzError>,
    pub error: Option<String>,
    pub function: Option<String>,
    pub id: Option<String>,
    pub negative_test: bool,       // Defaults to false.
    pub not_a_senzing_error: bool, // Defaults to false.
    pub reason: Option<String>,
    pub return_message: Option<String>,
}

pub fn get_testcases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "None",
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message, No reason",
            error_message: Some("".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message, empty reason",
            error_message: Some("".to_string()),
            reason: Some("".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in direct json",
            error_message: Some(r#"rpc error: code = Unknown desc = {"reason": "SZSDK00010001"}"#.to_string()),
            reason: Some("SZSDK00010001".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in nested json",
            error_message: Some(r#"rpc error: code = Unknown desc = {"error": {"reason": "SZSDK00020002"}}"#.to_string()),
            reason: Some("SZSDK00020002".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in no json",
            error_message: Some("rpc error: code = Unknown desc = plain text error".to_string()),
            reason: Some("SZSDK00020002".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason json without reason field",
            error_message: Some(r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#.to_string()),
            reason: Some("SZSDK00020002".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "No JSON message",
            error_message: Some("No JSON message".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "SzConfigurationError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzConfigurationError),
            error_type_parent: Some(SzError::SzGeneralError),
            error_id: Some(60),
            ..Default::default()
        },
        TestCase {
            name: "SzBadInputError",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("SENZ3131|Invalid column [BAD] requested for CSV export.".to_string()),
            error_type: Some(SzError::SzBadInputError),
            error_id: Some(3131),
            ..Default::default()
        },
        TestCase {
            name: "gRPC",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": {\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0057|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0057|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzError),
            error_id: Some(57),
            ..Default::default()
        },
        TestCase {
            name: "NegativeTest",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("SENZ3132|Invalid column [BAD] requested for CSV export.".to_string()), // Wrong SENZnnnn number
            error_type: Some(SzError::SzConfigurationError), // Wrong error_type
            error_id: Some(3132), // Wrong ID
            negative_test: true,
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All None",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All Some",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("".to_string()),
            error_type: Some(SzError::SzError),
            error_id: Some(0),
            not_a_senzing_error: true,
            ..Default::default()
        },
    ]
}

pub fn mock_senzing_function(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(error_message) = testcase.error_message {
        Err(Box::new(std::io::Error::other(error_message)))
    } else if let Some(return_message) = testcase.return_message {
        Ok(return_message)
    } else {
        Err(Box::new(std::io::Error::other("Not a SzError")))
    }
}

mod test {
    use super::{get_testcases, mock_senzing_function};
    use crate::error::SzError;
    use crate::error::{as_senzing_error, extract_reason_from_json};
    use crate::senzing_error_type1;
    use crate::senzing_error_type2;
    use crate::senzing_error_type3;
    use serde_json::Value;

    // ------------------------------------------------------------------------
    // Test simplified prototypes of error type
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

    // #[test]
    // fn test_match_senzing_error_type4() {
    //     let target = SzError::SzNotFoundError;

    //     match target {
    //         x if is_senzing_error_type(x, SzError::SzBadInputError) => {
    //             println!(">>>>>> match: Is SzBadInputError")
    //         }
    //         x if is_senzing_error_type(x, SzError::SzGeneralError) => {
    //             println!(">>>>>> match: Is SzGeneralError")
    //         }
    //         x if is_senzing_error_type(x, SzError::SzRetryableError) => {
    //             println!(">>>>>> match: Is SzRecoverableError")
    //         }
    //         x if is_senzing_error_type(x, SzError::SzUnrecoverableError) => {
    //             println!(">>>>>> match: Is SzUnrecoverableError")
    //         }
    //         x if is_senzing_error_type(x, SzError::SzError) => {
    //             println!(">>>>>> match: Is SzError")
    //         }
    //         _ => {
    //             println!(">>>>>> match: All other errors")
    //         }
    //     }
    // }

    // ------------------------------------------------------------------------
    // Using as_senzing_error()
    // ------------------------------------------------------------------------

    #[test]
    fn test_senzing_reasons() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(testcase_error_message) = testcase.error_message
                && let Some(testcase_reason) = testcase.reason
            {
                let senzing_error = as_senzing_error(testcase_error_message);
                if let Some(senzing_reason) = senzing_error.reason() {
                    if testcase.negative_test {
                        assert_ne!(
                            testcase_reason, senzing_reason,
                            "testcase={}; negative-testcase",
                            testcase.name
                        )
                    } else {
                        assert_eq!(
                            testcase_reason, senzing_reason,
                            "testcase={}",
                            testcase.name
                        )
                    }
                } else {
                    println!("    error_type() returned None");
                }
            } else {
                println!("    test ignored");
            }
        }
    }

    #[test]
    fn test_senzing_error_types_1() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            if let Some(testcase_error_message) = testcase.error_message
                && let Some(testcase_error_type) = testcase.error_type
            {
                let senzing_error = as_senzing_error(testcase_error_message);
                if let Some(senzing_error_type) = senzing_error.error_type() {
                    if testcase.negative_test {
                        assert_ne!(
                            testcase_error_type, senzing_error_type,
                            "testcase={}; negative-testcase",
                            testcase.name
                        )
                    } else {
                        assert_eq!(
                            testcase_error_type, senzing_error_type,
                            "testcase={}",
                            testcase.name
                        )
                    }
                } else {
                    println!("    error_type() returned None");
                }
            } else {
                println!("    test ignored");
            }
        }
    }

    // ------------------------------------------------------------------------
    // Using mock_senzing_function()
    // ------------------------------------------------------------------------

    // This technique forces exhaustive matches of Senzing error types.
    #[test]
    fn test_senzing_error_types_2() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let error_type_parent = testcase.error_type_parent;
            let testcase_name = testcase.name;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(senzing_error) => {
                    let senzing_error = as_senzing_error(senzing_error);
                    if let Some(sz) = senzing_error.error_type() {
                        match sz {
                            senzing_error_type1!(SzError::SzBadInputError) => {
                                if let Some(expected) = error_type_parent {
                                    assert_eq!(
                                        expected,
                                        SzError::SzBadInputError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            senzing_error_type1!(SzError::SzGeneralError) => {
                                if let Some(expected) = error_type_parent {
                                    assert_eq!(
                                        expected,
                                        SzError::SzGeneralError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            senzing_error_type1!(SzError::SzRetryableError) => {
                                if let Some(expected) = error_type_parent {
                                    assert_eq!(
                                        expected,
                                        SzError::SzRetryableError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            senzing_error_type1!(SzError::SzUnrecoverableError) => {
                                if let Some(expected) = error_type_parent {
                                    assert_eq!(
                                        expected,
                                        SzError::SzUnrecoverableError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            senzing_error_type1!(SzError::SzError) => {
                                if let Some(expected) = error_type_parent {
                                    assert_eq!(
                                        expected,
                                        SzError::SzError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // This technique forces exhaustive matches of Senzing error types.
    #[test]
    fn test_senzing_error_types_3() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let error_type_parent = testcase.error_type_parent;
            let not_a_senzing_error = testcase.not_a_senzing_error;
            let testcase_name = testcase.name;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(result_error) => match as_senzing_error(result_error).error_type() {
                    Some(senzing_error_type1!(SzError::SzBadInputError)) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzBadInputError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    Some(senzing_error_type1!(SzError::SzGeneralError)) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzGeneralError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    Some(senzing_error_type1!(SzError::SzRetryableError)) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzRetryableError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    Some(senzing_error_type1!(SzError::SzUnrecoverableError)) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzUnrecoverableError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    Some(senzing_error_type1!(SzError::SzError)) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(expected, SzError::SzError, "testcase={}", testcase_name);
                        }
                    }
                    // Some(_) => {
                    //     println!("    Uncaught senzing error")
                    // }
                    None => {
                        assert!(
                            not_a_senzing_error,
                            "testcase={} - not a senzing error",
                            testcase_name
                        );
                    }
                },
            }
        }
    }

    #[test]
    fn test_senzing_error_types_4() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let error_type_parent = testcase.error_type_parent;
            let not_a_senzing_error = testcase.not_a_senzing_error;
            let testcase_name = testcase.name;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(result_error) => match as_senzing_error(result_error) {
                    x if x.is_error_type(SzError::SzBadInputError) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzBadInputError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    x if x.is_error_type(SzError::SzGeneralError) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzGeneralError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    x if x.is_error_type(SzError::SzRetryableError) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzRetryableError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    x if x.is_error_type(SzError::SzUnrecoverableError) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(
                                expected,
                                SzError::SzUnrecoverableError,
                                "testcase={}",
                                testcase_name
                            );
                        }
                    }
                    x if x.is_error_type(SzError::SzError) => {
                        if let Some(expected) = error_type_parent {
                            assert_eq!(expected, SzError::SzError, "testcase={}", testcase_name);
                        }
                    }
                    _ => {
                        assert!(
                            not_a_senzing_error,
                            "testcase={} - not a senzing error",
                            testcase_name
                        );
                    }
                },
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

    // ------------------------------------------------------------------------
    // Using mock_senzing_function()
    // ------------------------------------------------------------------------

    // #[test]
    // fn test_reason_from_direct_json() {
    //     let error =
    //         as_senzing_error(r#"rpc error: code = Unknown desc = {"reason": "SZSDK00010001"}"#);
    //     assert_eq!(error.reason(), Some("SZSDK00010001".to_string()));
    // }

    // #[test]
    // fn test_reason_from_nested_json() {
    //     let error = as_senzing_error(
    //         r#"rpc error: code = Unknown desc = {"error": {"reason": "SZSDK00020002"}}"#,
    //     );
    //     assert_eq!(error.reason(), Some("SZSDK00020002".to_string()));
    // }

    // #[test]
    // fn test_reason_no_json() {
    //     let error = as_senzing_error("rpc error: code = Unknown desc = plain text error");
    //     assert_eq!(error.reason(), None);
    // }

    // #[test]
    // fn test_reason_json_without_reason_field() {
    //     let error = as_senzing_error(
    //         r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#,
    //     );
    //     assert_eq!(error.reason(), None);
    // }

    // ------------------------------------------------------------------------
    // Using serde_json for targeted tests.
    // ------------------------------------------------------------------------

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

    // #[test]
    // fn test_reason_from_grpc_error_with_escaped_json() {
    //     // This tests the actual format we see from gRPC errors with deeply nested reason
    //     let error_msg = r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": {\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0057|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#;
    //     let error = as_senzing_error(error_msg);
    //     assert_eq!(
    //         error.reason(),
    //         Some("SENZ0057|Unknown feature ID value '1'".to_string())
    //     );
    // }
}
