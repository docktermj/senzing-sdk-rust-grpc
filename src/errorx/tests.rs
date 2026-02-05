#[cfg(test)]
use crate::errorx::{SenzingError, SzError, SzErrorTrait, extract_senzing_error};

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
            name: "No tests",
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message and no reason",
            error_message: Some("".to_string()),
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Empty message and empty reason",
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
            not_a_senzing_error: true,
            ..Default::default()
        },
        TestCase {
            name: "Reason json without reason field",
            error_message: Some(r#"rpc error: code = Unknown desc = {"message": "something went wrong"}"#.to_string()),
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
            name: "SzError",
            error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ0005|Exceeded the Maximum Number of Retries Allowed"}}}}"#.to_string()),
            reason: Some("SENZ0005|Exceeded the Maximum Number of Retries Allowed".to_string()),
            error_type: Some(SzError::SzError),
            error_type_parent: Some(SzError::SzError),
            error_id: Some(5),
            ..Default::default()
        },
        // TestCase {
        //     name: "SzBadInputError",
        //     error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
        //     reason: Some("SENZ3131|Invalid column [BAD] requested for CSV export.".to_string()),
        //     error_type: Some(SzError::SzBadInputError),
        //     error_id: Some(3131),
        //     ..Default::default()
        // },
        TestCase {
            name: "SzNotFoundError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0033|Unknown record: dsrc[{0}], record[{1}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0033|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzNotFoundError),
            error_type_parent: Some(SzError::SzBadInputError),
            error_id: Some(33),
            ..Default::default()
        },
        TestCase {
            name: "SzUnknownDataSourceError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ2207|Data source code [{0}] does not exist.\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ2207|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzUnknownDataSourceError),
            error_type_parent: Some(SzError::SzBadInputError),
            error_id: Some(2207),
            ..Default::default()
        },
        // TestCase {
        //     name: "SzGeneralError",
        //     error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
        //     reason: Some("SENZ0060|Unknown feature ID value '1'".to_string()),
        //     error_type: Some(SzError::SzGeneralError),
        //     error_type_parent: Some(SzError::SzGeneralError),
        //     error_id: Some(60),
        //     ..Default::default()
        // },
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
            name: "SzReplaceConflictError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ7245|Current configuration ID does not match specified data ID [{0}].\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ7245|Unknown feature ID value '1'".to_string()),
            error_type: Some(SzError::SzReplaceConflictError),
            error_type_parent: Some(SzError::SzGeneralError),
            error_id: Some(7245),
            ..Default::default()
        },
        // TestCase {
        //     name: "SzRetryableError",
        //     error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ7245|Current configuration ID does not match specified data ID [{0}].\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
        //     reason: Some("SENZ7245|Unknown feature ID value '1'".to_string()),
        //     error_type: Some(SzError::SzRetryableError),
        //     error_type_parent: Some(SzError::SzRetryableError),
        //     error_id: Some(7245),
        //     ..Default::default()
        // },
        TestCase {
            name: "SzDatabaseConnectionLostError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ1006|Database Connection Failure '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ1006|Database Connection Failure '{0}'".to_string()),
            error_type: Some(SzError::SzDatabaseConnectionLostError),
            error_type_parent: Some(SzError::SzRetryableError),
            error_id: Some(1006),
            ..Default::default()
        },
        TestCase {
            name: "SzDatabaseTransientError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ1008|Deadlock Error '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ1008|Deadlock Error '{0}'".to_string()),
            error_type: Some(SzError::SzDatabaseTransientError),
            error_type_parent: Some(SzError::SzRetryableError),
            error_id: Some(1008),
            ..Default::default()
        },
        TestCase {
            name: "SzRetryTimeoutExceededError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]".to_string()),
            error_type: Some(SzError::SzRetryTimeoutExceededError),
            error_type_parent: Some(SzError::SzRetryableError),
            error_id: Some(10),
            ..Default::default()
        },
        // TestCase {
        //     name: "SzUnrecoverableError",
        //     error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"Retry timeout exceeded resolved entity locklist [{0}]\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
        //     reason: Some("SENZ0010|Retry timeout exceeded resolved entity locklist [{0}]".to_string()),
        //     error_type: Some(SzError::SzUnrecoverableError),
        //     error_type_parent: Some(SzError::SzUnrecoverableError),
        //     error_id: Some(10),
        //     ..Default::default()
        // },
        TestCase {
            name: "SzDatabaseError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0054|Data repository was purged\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0054|Data repository was purged".to_string()),
            error_type: Some(SzError::SzDatabaseError),
            error_type_parent: Some(SzError::SzUnrecoverableError),
            error_id: Some(54),
            ..Default::default()
        },
        TestCase {
            name: "SzLicenseError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0999|License has expired. {0}\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0999|License has expired. {0}".to_string()),
            error_type: Some(SzError::SzLicenseError),
            error_type_parent: Some(SzError::SzUnrecoverableError),
            error_id: Some(999),
            ..Default::default()
        },
        TestCase {
            name: "SzNotInitializedError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0048|SDK is not initialized\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0048|SDK is not initialized".to_string()),
            error_type: Some(SzError::SzNotInitializedError),
            error_type_parent: Some(SzError::SzUnrecoverableError),
            error_id: Some(48),
            ..Default::default()
        },
        TestCase {
            name: "SzUnhandledError",
            error_message: Some(r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": \n{\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0087|Sz Exception '{0}'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string()),
            reason: Some("SENZ0087|Sz Exception '{0}'".to_string()),
            error_type: Some(SzError::SzUnhandledError),
            error_type_parent: Some(SzError::SzUnrecoverableError),
            error_id: Some(87),
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
        // TestCase {
        //     name: "NegativeTest",
        //     error_message: Some(r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string()),
        //     reason: Some("SENZ3132|Invalid column [BAD] requested for CSV export.".to_string()), // Wrong SENZnnnn number
        //     error_type: Some(SzError::SzConfigurationError), // Wrong error_type
        //     error_id: Some(3132), // Wrong ID
        //     negative_test: true,
        //     ..Default::default()
        // },
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
        Err(SenzingError::new(error_message))
    } else if let Some(return_message) = testcase.return_message {
        Ok(return_message)
    } else {
        Err(Box::new(std::io::Error::other("Not a SzError")))
    }
}

// pub fn mock_senzing_function(testcase: TestCase) -> Result<String, Box<dyn std::error::Error>> {
//     if let Some(error_message) = testcase.error_message {
//         Err(Box::new(std::io::Error::other(error_message)))
//     } else if let Some(return_message) = testcase.return_message {
//         Ok(return_message)
//     } else {
//         Err(Box::new(std::io::Error::other("Not a SzError")))
//     }
// }

pub fn mock_senzing_function_error() -> Result<String, Box<dyn std::error::Error>> {
    let senzing_error: SenzingError<crate::errorx::SzBadInputError> = SenzingError {
        message: "The Senzing error".to_string(),
        error_type: Default::default(),
        error_hierarchy: vec![],
        state: std::marker::PhantomData,
    };
    // Err(Box::new(std::io::Error::other(senzing_error)))
    Err(Box::new(senzing_error))
}

pub fn new_error() -> Box<dyn SzErrorTrait> {
    SenzingError::new("Just a message".to_string())
}

// pub fn new_error_2() -> Box<dyn SzErrorTrait> {
//     SenzingError::new("Just a message".to_string())
// }

// pub fn new_error_3() -> Box<dyn SzErrorTrait> {
//     SenzingError::new("Just a message".to_string())
// }

mod test {
    use std::boxed;

    use super::{get_testcases, mock_senzing_function};

    use crate::errorx::SzError;
    use crate::errorx::{SenzingError, SzDatabaseError, SzErrorTrait};

    // ------------------------------------------------------------------------
    // Using mock_senzing_function()
    // ------------------------------------------------------------------------

    // This technique forces exhaustive matches of Senzing error types.
    #[test]
    fn test_senzing_error_types_2() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let testcase_name = testcase.name;
            let testcase_error_type = testcase.error_type;
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(boxed_senzing_error) => {
                    if let Ok(sz_error) = boxed_senzing_error.downcast::<SenzingError>() {
                        match sz_error.error_type() {
                            SzError::SzBadInputError => {
                                if let Some(expected) = testcase_error_type {
                                    assert_eq!(
                                        expected,
                                        SzError::SzBadInputError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            SzError::SzDatabaseError => {
                                let bill = sz_error.downcast::<SenzingError<SzDatabaseError>>();
                                if let Some(bill_x) = bill {
                                    bill_x.mjd_was_here("".to_string());
                                }
                                if let Some(expected) = testcase_error_type {
                                    assert_eq!(
                                        expected,
                                        SzError::SzGeneralError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            SzError::SzGeneralError => {
                                if let Some(expected) = testcase_error_type {
                                    assert_eq!(
                                        expected,
                                        SzError::SzGeneralError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            SzError::SzRetryableError => {
                                if let Some(expected) = testcase_error_type {
                                    assert_eq!(
                                        expected,
                                        SzError::SzRetryableError,
                                        "testcase={}",
                                        testcase_name
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_senzing_error_types_3() {
        let testcases = get_testcases();
        for testcase in testcases {
            println!("{}", testcase.name);
            let testcase_name = testcase.name;
            let testcase_error_type_parent = testcase.error_type_parent.unwrap_or(SzError::SzError);
            let senzing_result = mock_senzing_function(testcase);
            match senzing_result {
                Ok(senzing_message) => {
                    println!("    Message: {}", senzing_message);
                }
                Err(err) => {
                    if let Some(senzing_error) = extract_senzing_error!(err) {
                        // println!("    >>>>>> error_type: {:?}", senzing_error.error_type());

                        match senzing_error {
                            x if x.is(SzError::SzDatabaseError) => {}
                            _ => {}
                        }

                        if senzing_error.is(SzError::SzBadInputError) {
                            println!("    >>>>>> testing: SzBadInputError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzBadInputError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzBadInputError,
                                "testcase={}",
                                testcase_name
                            );
                        } else if senzing_error.is(SzError::SzDatabaseError) {
                            println!("    >>>>>> testing: SzDatabaseError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzDatabaseError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzUnrecoverableError,
                                "testcase={}",
                                testcase_name
                            );
                        } else if senzing_error.is(SzError::SzGeneralError) {
                            println!("    >>>>>> testing: SzGeneralError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzGeneralError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzGeneralError,
                                "testcase={}",
                                testcase_name
                            );
                        } else if senzing_error.is(SzError::SzRetryableError) {
                            println!("    >>>>>> testing: SzRetryableError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzRetryableError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzRetryableError,
                                "testcase={}",
                                testcase_name
                            );
                        } else if senzing_error.is(SzError::SzUnrecoverableError) {
                            println!("    >>>>>> testing: SzUnrecoverableError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzUnrecoverableError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzUnrecoverableError,
                                "testcase={}",
                                testcase_name
                            );
                        } else if senzing_error.is(SzError::SzError) {
                            println!("    >>>>>> testing: SzError");
                            if let Some(new_err) =
                                err.downcast_ref::<SenzingError<SzDatabaseError>>()
                            {
                                new_err.mjd_was_here("SzError".to_string());
                            }
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzError,
                                "testcase={}",
                                testcase_name
                            );
                        } else {
                            println!("    >>>>>> testing: No Parent");
                            assert_eq!(
                                testcase_error_type_parent,
                                SzError::SzError,
                                "testcase={} - else",
                                testcase_name
                            );
                        }
                    } else {
                        println!("    >>>>>> Not a SenzingError. (downcast failed)")
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Test SenzingError::new() returns different variants
    // ------------------------------------------------------------------------

    // #[test]
    // fn test_senzing_error_new_returns_different_variants() {
    //     use crate::errorx::SzError;

    //     // Test 1: SzConfigurationError variant (error code 60)
    //     let config_error_msg = r#"status: 'Unknown error', self: "{\"function\": \"szdiagnosticserver.(*SzDiagnosticServer).GetFeature\", \"error\": {\"function\": \"szdiagnostic.(*Szdiagnostic).GetFeature\", \"error\": {\"id\":\"SZSDK60034004\",\"reason\":\"SENZ0060|Unknown feature ID value '1'\"}}}", metadata: {"content-type": "application/grpc"}"#.to_string();
    //     let err1 = SenzingError::new(config_error_msg.clone());

    //     println!("Test 1: SzConfigurationError");
    //     println!("  Message: {}", config_error_msg);
    //     println!("  Error type: {:?}", err1.error_type());
    //     println!("  Expected: SzConfigurationError");
    //     assert_eq!(
    //         err1.error_type(),
    //         SzError::SzConfigurationError,
    //         "Expected SzConfigurationError for SENZ0060 error code"
    //     );

    //     // Test 2: SzBadInputError variant (error code 3131)
    //     let bad_input_error_msg = r#"{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"function":"szengineserver.(*SzEngineServer).ExportCsvEntityReport","error":{"function":"szengine.(*Szengine).ExportCsvEntityReport","error":{"id":"SZSDK60044007","reason":"SENZ3131|Invalid column [BAD] requested for CSV export."}}}}"#.to_string();
    //     let err2 = SenzingError::new(bad_input_error_msg.clone());

    //     println!("\nTest 2: SzBadInputError");
    //     println!("  Message: {}", bad_input_error_msg);
    //     println!("  Error type: {:?}", err2.error_type());
    //     println!("  Expected: SzBadInputError");
    //     assert_eq!(
    //         err2.error_type(),
    //         SzError::SzBadInputError,
    //         "Expected SzBadInputError for SENZ3131 error code"
    //     );

    //     // Test 3: Generic SzError variant (no error code)
    //     let generic_error_msg = "Just a plain message with no error code".to_string();
    //     let err3 = SenzingError::new(generic_error_msg.clone());

    //     println!("\nTest 3: Generic SzError (no error code)");
    //     println!("  Message: {}", generic_error_msg);
    //     println!("  Error type: {:?}", err3.error_type());
    //     println!("  Expected: SzError");
    //     assert_eq!(
    //         err3.error_type(),
    //         SzError::SzError,
    //         "Expected generic SzError for message without error code"
    //     );

    //     println!("\n✓ All variants correctly created by SenzingError::new()");
    // }

    // #[test]
    // fn test_match_senzing_error_type_10() {
    //     let err = new_error();
    //     println!(">>>> {:?}", err);
    // }

    // ------------------------------------------------------------------------
    // Test simplified prototypes of error type
    // ------------------------------------------------------------------------

    // #[test]
    // fn test_match_senzing_error_type_1() {
    //     let result = mock_senzing_function_error();
    //     match result {
    //         Ok(string) => {
    //             println!(">>>>>> {}", string);
    //         }
    //         Err(err) => {
    //             println!(">>>>>> In error: {:?}", err);
    //             if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
    //                 println!(">>>>>> Senzing Err: {}", sz)
    //             } else if let Some(sz) = err.downcast_ref::<SenzingError<SzDatabaseError>>() {
    //                 println!(">>>>>> Senzing Err: {}", sz)
    //             } else {
    //                 println!(">>>>>> Non-Senzing")
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_match_senzing_error_type_2() {
    //     let result = mock_senzing_function_error();
    //     match result {
    //         Ok(string) => {
    //             println!(">>>>>> {}", string);
    //         }
    //         Err(err) => {
    //             println!(">>>>>> In error: {:?}", err);
    //             if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
    //                 println!(">>>>>> Senzing Err: {}", sz)
    //             } else if let Some(sz) = err.downcast_ref::<SenzingError<SzDatabaseError>>() {
    //                 sz.mjd_was_here();

    //                 println!(">>>>>> Senzing Err: {}", sz)
    //             } else {
    //                 println!(">>>>>> Non-Senzing")
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_match_senzing_error_type_2() {
    //     let result = mock_senzing_function_error();
    //     match result {
    //         Ok(string) => {
    //             println!(">>>>>> {}", string);
    //         }
    //         Err(err) => {
    //             println!(">>>>>> In error: {:?}", err);
    //             // Try downcasting to SenzingError<SzBadInputError> first
    //             if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
    //                 println!(">>>>>> Senzing Err (SzBadInputError): {}", sz);
    //                 sz.for_all();
    //             } else if let Some(sz) = err.downcast_ref::<SenzingError>() {
    //                 // Try downcasting to default SenzingError (i.e., SenzingError<SzErrorX>)
    //                 println!(">>>>>> Senzing Err (default): {}", sz);
    //                 sz.for_all();
    //             } else {
    //                 println!(">>>>>> Non-Senzing")
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_match_senzing_error_type_3() {
    //     // Helper enum to enable match syntax for different error types
    //     enum ErrorType<'a> {
    //         BadInput(&'a SenzingError<SzBadInputError>),
    //         License(&'a SenzingError<SzLicenseError>),
    //         RetryTimeout(&'a SenzingError<SzRetryTimeoutExceededError>),
    //         Other,
    //     }

    //     let result = mock_senzing_function_error();
    //     match result {
    //         Ok(string) => {
    //             println!(">>>>>> {}", string);
    //         }
    //         Err(err) => {
    //             println!(">>>>>> In error: {:?}", err);

    //             // Determine which error type we have
    //             let error_type =
    //                 if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
    //                     ErrorType::BadInput(sz)
    //                 } else if let Some(sz) = err.downcast_ref::<SenzingError<SzLicenseError>>() {
    //                     ErrorType::License(sz)
    //                 } else if let Some(sz) =
    //                     err.downcast_ref::<SenzingError<SzRetryTimeoutExceededError>>()
    //                 {
    //                     ErrorType::RetryTimeout(sz)
    //                 } else {
    //                     ErrorType::Other
    //                 };

    //             // Now we can use match to differentiate
    //             match error_type {
    //                 ErrorType::BadInput(sz) => {
    //                     println!(">>>>>> Senzing Err (SzBadInputError): {}", sz);
    //                 }
    //                 ErrorType::License(sz) => {
    //                     println!(">>>>>> Senzing Err (SzLicenseError): {}", sz);
    //                 }
    //                 ErrorType::RetryTimeout(sz) => {
    //                     println!(">>>>>> Senzing Err (SzRetryTimeoutExceededError): {}", sz);
    //                 }
    //                 ErrorType::Other => {
    //                     println!(">>>>>> Non-Senzing or unknown error type");
    //                 }
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_match_senzing_error_type_4() {
    //     let result = mock_senzing_function_error();
    //     match result {
    //         Ok(string) => {
    //             println!(">>>>>> {}", string);
    //         }
    //         Err(err) => {
    //             println!(">>>>>> In error: {:?}", err);

    //             if let Some(senzing_error) = err.downcast_ref::<SenzingError>() {
    //                 let senzing_error_type = senzing_error.error_type();
    //             } else {
    //                 println!(">>>>>> Non-Senzing or unknown error type");
    //             }
    //         }
    //     }
    // }
}
