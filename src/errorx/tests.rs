#[cfg(test)]
use crate::errorx::{SzError, SzErrorTypes};

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

mod test {
    use super::{get_testcase, get_testcases, mock_senzing_function};
    use crate::errorx::{
        SzDatabaseError, SzError, SzErrorTypes, SzNotFoundError, is_normal,
    };
    use crate::extract_senzing_error;

    #[test]
    fn test_trait_default_default() {
        let test_senzing_error = SzError::<SzDatabaseError>::default();
        println!("Default: {}", test_senzing_error)
    }

    #[test]
    fn test_trait_display_fmt() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(err) => {
                    if let Some(senzing_error) = extract_senzing_error!(err) {
                        println!("    {}", senzing_error)
                    }
                }
            }
        }
    }

    #[test]
    fn test_trait_display_mjd() {
        let testcase = get_testcase();
        let senzing_result = mock_senzing_function(testcase);
        match senzing_result {
            Ok(senzing_message) => {
                println!("    Message: {}", senzing_message);
            }
            // Err(ref err) if err.downcast_ref::<SzError<SzNotFoundError>>().is_some() => {}
            Err(ref e) if SzError::error_is(e, SzError::<SzNotFoundError>::default()) => {/* handle */}
            Err(e) if e.to_string().contains("timeout") => { /* handle timeout */ }
            Err(e) if e.to_string().contains("connection") => { /* handle connection error */ }
            // Err(err) if extract_senzing_error!(err) => {}
            Err(err) if SzError::is_senzing_retryable_error(&err) => { /* handle */ }
            Err(err) if SzError::is_senzing_error(&err) => { /* handle */ }
            Err(_) => {} // Err(_) => { /* handle other errors */ }
        }
    }

    #[test]
    fn test_senzing_error_types_using_if_else() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let testcase_name = testcase.name;
            let testcase_error_type_parent = testcase.error_type_parent;
            let is_not_a_senzing_error = testcase.is_not_a_senzing_error;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(err) => {
                    if let Some(expected_error_type_parent) = testcase_error_type_parent {
                        if let Some(senzing_error) = extract_senzing_error!(err) {
                            if senzing_error.kind(SzErrorTypes::BadInputError) {
                                println!("    is an SzBadInputError");
                                assert_eq!(
                                    expected_error_type_parent,
                                    SzErrorTypes::BadInputError,
                                    "testcase={}",
                                    testcase_name
                                );
                            } else if senzing_error.kind(SzErrorTypes::GeneralError) {
                                println!("    is an SzGeneralError");
                                assert_eq!(
                                    expected_error_type_parent,
                                    SzErrorTypes::GeneralError,
                                    "testcase={}",
                                    testcase_name
                                );
                            } else if senzing_error.kind(SzErrorTypes::RetryableError) {
                                println!("    is an SzRetryableError");
                                assert_eq!(
                                    expected_error_type_parent,
                                    SzErrorTypes::RetryableError,
                                    "testcase={}",
                                    testcase_name
                                );
                            } else if senzing_error.kind(SzErrorTypes::UnrecoverableError) {
                                println!("    is an SzUnrecoverableError");
                                if let Some(new_err) =
                                    err.downcast_ref::<SzError<SzDatabaseError>>()
                                {
                                    new_err.mjd_was_here("SzUnrecoverableError".to_string());
                                }
                                assert_eq!(
                                    expected_error_type_parent,
                                    SzErrorTypes::UnrecoverableError,
                                    "testcase={}",
                                    testcase_name
                                );
                            } else if senzing_error.kind(SzErrorTypes::Error) {
                                println!("    is an SzError");
                                assert_eq!(
                                    expected_error_type_parent,
                                    SzErrorTypes::Error,
                                    "testcase={}",
                                    testcase_name
                                );
                            } else {
                                unreachable!("testcase={} - else", testcase_name);
                            }
                        } else {
                            assert!(is_not_a_senzing_error, "testcase={}", testcase_name)
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_senzing_error_types_using_match() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let testcase_name = testcase.name;
            let testcase_error_type_parent = testcase.error_type_parent;
            let is_not_a_senzing_error = testcase.is_not_a_senzing_error;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(err) => {
                    if let Some(expected_error_type_parent) = testcase_error_type_parent {
                        if let Some(senzing_error) = extract_senzing_error!(err) {
                            match senzing_error {
                                x if x.kind(SzErrorTypes::BadInputError) => {
                                    assert_eq!(
                                        expected_error_type_parent,
                                        SzErrorTypes::BadInputError,
                                        "testcase={}",
                                        testcase_name
                                    )
                                }
                                x if x.kind(SzErrorTypes::GeneralError) => {
                                    assert_eq!(
                                        expected_error_type_parent,
                                        SzErrorTypes::GeneralError,
                                        "testcase={}",
                                        testcase_name
                                    )
                                }
                                x if x.kind(SzErrorTypes::RetryableError) => {
                                    assert_eq!(
                                        expected_error_type_parent,
                                        SzErrorTypes::RetryableError,
                                        "testcase={}",
                                        testcase_name
                                    )
                                }
                                x if x.kind(SzErrorTypes::UnrecoverableError) => {
                                    assert_eq!(
                                        expected_error_type_parent,
                                        SzErrorTypes::UnrecoverableError,
                                        "testcase={}",
                                        testcase_name
                                    )
                                }
                                x if x.kind(SzErrorTypes::Error) => {
                                    assert_eq!(
                                        expected_error_type_parent,
                                        SzErrorTypes::Error,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                                _ => {
                                    unreachable!("testcase={} - else", testcase_name)
                                }
                            }
                        } else {
                            assert!(is_not_a_senzing_error, "testcase={}", testcase_name)
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[allow(path_statements)]
    fn test_is_normal_type() {
        is_normal::<SzError>;
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

pub fn mock_senzing_function(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(error_message) = testcase.error_message {
        Err(SzError::new(error_message))
    } else if let Some(return_message) = testcase.return_message {
        Ok(return_message)
    } else {
        Err(Box::new(std::io::Error::other("Not a SzError")))
    }
}

// pub fn throw_error_level_1(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
//     throw_error_level_2(testcase)?;
//     Ok("".to_string())
// }

// pub fn throw_error_level_2(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
//     throw_error_level_3(testcase)?;
//     Ok("".to_string())
// }

// pub fn throw_error_level_3(testcase: TestCase) -> Result<String, Box<dyn SzErrorTrait>> {
//     if let Some(error_message) = testcase.error_message {
//         return Err(SzError::new(error_message))
//     }
//     Ok("no message".to_string())
// }

// pub fn throw_error_level_3(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
//     if let Some(error_message) = testcase.error_message {
//         Err(SzError::new(error_message))
//     } else if let Some(return_message) = testcase.return_message {
//         Ok(return_message)
//     } else {
//         Err(Box::new(std::io::Error::other("Not a SzError")))
//     }
// }

// ----------------------------------------------------------------------------
// Testcase data
// ----------------------------------------------------------------------------

#[derive(Debug, Default, PartialEq)]
pub struct TestCase {
    pub name: &'static str,
    pub error_id: Option<i32>,
    pub error_message: Option<String>,
    pub error_type_parent: Option<SzErrorTypes>,
    pub error_type: Option<SzErrorTypes>,
    pub error: Option<String>,
    pub function: Option<String>,
    pub id: Option<String>,
    pub is_a_negative_test: bool,     // Defaults to false.
    pub is_not_a_senzing_error: bool, // Defaults to false.
    pub reason: Option<String>,
    pub return_message: Option<String>,
}

pub fn get_testcase() -> TestCase {
    TestCase {
            name: "SzBadInputError",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("SENZ3131|Invalid column [BAD] requested for CSV export.".to_string()),
            error_type: Some(SzErrorTypes::BadInputError),
            error_id: Some(3131),
            ..Default::default()
        }
}

pub fn get_testcases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "No tests",
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message and no reason",
            error_message: Some("".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message and empty reason",
            error_message: Some("".to_string()),
            reason: Some("".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in direct json",
            error_message: Some(r#"rpc error: code = Unknown desc = {"reason": "SZSDK00010001"}"#.to_string()),
            reason: Some("SZSDK00010001".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in nested json",
            error_message: Some(r#"rpc error: code = Unknown desc = {"error": {"reason": "SZSDK00020002"}}"#.to_string()),
            reason: Some("SZSDK00020002".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason in no json",
            error_message: Some("rpc error: code = Unknown desc = plain text error".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason json without reason field",
            error_message: Some(r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#.to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "No JSON message",
            error_message: Some("No JSON message".to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "SzError",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ0005|Exceeded the Maximum Number of Retries Allowed"}}}}"#.to_string()),
            reason: Some("SENZ0005|Exceeded the Maximum Number of Retries Allowed".to_string()),
            error_type: Some(SzErrorTypes::Error),
            error_id: Some(5),
            ..Default::default()
        },
        TestCase {
            name: "SzBadInputError",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("SENZ3131|Invalid column [BAD] requested for CSV export.".to_string()),
            error_type: Some(SzErrorTypes::BadInputError),
            error_id: Some(3131),
            ..Default::default()
        },
        TestCase {
            name: "SzNotFoundError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0033|Unknown record: dsrc[{0}], record[{1}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0033|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::NotFoundError),
            error_type_parent: Some(SzErrorTypes::BadInputError),
            error_id: Some(33),
            ..Default::default()
        },
        TestCase {
            name: "SzUnknownDataSourceError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ2207|Data source code [{0}] does not exist.\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ2207|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::UnknownDataSourceError),
            error_type_parent: Some(SzErrorTypes::BadInputError),
            error_id: Some(2207),
            ..Default::default()
        },
        TestCase {
            name: "SzGeneralError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::GeneralError),
            error_id: Some(60),
            ..Default::default()
        },
        TestCase {
            name: "SzConfigurationError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::ConfigurationError),
            error_type_parent: Some(SzErrorTypes::GeneralError),
            error_id: Some(60),
            ..Default::default()
        },
        TestCase {
            name: "SzReplaceConflictError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ7245|Current configuration ID does not match specified data ID [{0}].\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ7245|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::ReplaceConflictError),
            error_type_parent: Some(SzErrorTypes::GeneralError),
            error_id: Some(7245),
            ..Default::default()
        },
        TestCase {
            name: "SzRetryableError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ7245|Current configuration ID does not match specified data ID [{0}].\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ7245|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::RetryableError),
            error_id: Some(7245),
            ..Default::default()
        },
        TestCase {
            name: "SzDatabaseConnectionLostError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ1006|Database Connection Failure '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ1006|Database Connection Failure '{0}'".to_string()),
            error_type: Some(SzErrorTypes::DatabaseConnectionLostError),
            error_type_parent: Some(SzErrorTypes::RetryableError),
            error_id: Some(1006),
            ..Default::default()
        },
        TestCase {
            name: "SzDatabaseTransientError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ1008|Deadlock Error '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ1008|Deadlock Error '{0}'".to_string()),
            error_type: Some(SzErrorTypes::DatabaseTransientError),
            error_type_parent: Some(SzErrorTypes::RetryableError),
            error_id: Some(1008),
            ..Default::default()
        },
        TestCase {
            name: "SzRetryTimeoutExceededError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]".to_string()),
            error_type: Some(SzErrorTypes::RetryTimeoutExceededError),
            error_type_parent: Some(SzErrorTypes::RetryableError),
            error_id: Some(10),
            ..Default::default()
        },
        TestCase {
            name: "SzUnrecoverableError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"Retry timeout exceeded resolved entity locklist [{0}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]".to_string()),
            error_type: Some(SzErrorTypes::UnrecoverableError),
            error_id: Some(10),
            ..Default::default()
        },
        TestCase {
            name: "SzDatabaseError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0054|Data repository was purged\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0054|Data repository was purged".to_string()),
            error_type: Some(SzErrorTypes::DatabaseError),
            error_type_parent: Some(SzErrorTypes::UnrecoverableError),
            error_id: Some(54),
            ..Default::default()
        },
        TestCase {
            name: "SzLicenseError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0999|License has expired. {0}\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0999|License has expired. {0}".to_string()),
            error_type: Some(SzErrorTypes::LicenseError),
            error_type_parent: Some(SzErrorTypes::UnrecoverableError),
            error_id: Some(999),
            ..Default::default()
        },
        TestCase {
            name: "SzNotInitializedError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0048|SDK is not initialized\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0048|SDK is not initialized".to_string()),
            error_type: Some(SzErrorTypes::NotInitializedError),
            error_type_parent: Some(SzErrorTypes::UnrecoverableError),
            error_id: Some(48),
            ..Default::default()
        },
        TestCase {
            name: "SzUnhandledError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0087|Sz Exception '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0087|Sz Exception '{0}'".to_string()),
            error_type: Some(SzErrorTypes::UnhandledError),
            error_type_parent: Some(SzErrorTypes::UnrecoverableError),
            error_id: Some(87),
            ..Default::default()
        },
        TestCase {
            name: "gRPC",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": {\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0057|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0057|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzErrorTypes::Error),
            error_id: Some(57),
            ..Default::default()
        },
        TestCase {
            name: "NegativeTest",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("SENZ3132|Invalid column [BAD] requested for CSV export.".to_string()), // Wrong SENZnnnn number
            error_type: Some(SzErrorTypes::ConfigurationError), // Wrong error_type
            error_id: Some(3132), // Wrong ID
            is_a_negative_test: true,
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All None",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "MalformedJSON - All Some",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
            reason: Some("".to_string()),
            error_type: Some(SzErrorTypes::Error),
            error_id: Some(0),
            is_not_a_senzing_error: true,
            ..Default::default()
        },
    ]
}
