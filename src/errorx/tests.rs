#[cfg(test)]
use crate::error::SzError;
use crate::errorx::SenzingError;

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

pub fn mock_senzing_function_error() -> Result<String, Box<dyn std::error::Error>> {
    let senzing_error: SenzingError<crate::errorx::SzBadInputError> = SenzingError {
        message: "The Senzing error".to_string(),
        state: std::marker::PhantomData,
    };
    // Err(Box::new(std::io::Error::other(senzing_error)))
    Err(Box::new(senzing_error))
}

mod test {
    use super::mock_senzing_function_error;

    use crate::errorx::{
        SenzingError, SzBadInputError, SzLicenseError, SzRetryTimeoutExceededError,
    };

    // ------------------------------------------------------------------------
    // Test simplified prototypes of error type
    // ------------------------------------------------------------------------

    #[test]
    fn test_match_senzing_error_type_1() {
        let result = mock_senzing_function_error();
        match result {
            Ok(string) => {
                println!(">>>>>> {}", string);
            }
            Err(err) => {
                println!(">>>>>> In error: {:?}", err);
                if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
                    println!(">>>>>> Senzing Err: {}", sz)
                } else {
                    println!(">>>>>> Non-Senzing")
                }
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type_2() {
        let result = mock_senzing_function_error();
        match result {
            Ok(string) => {
                println!(">>>>>> {}", string);
            }
            Err(err) => {
                println!(">>>>>> In error: {:?}", err);
                // Try downcasting to SenzingError<SzBadInputError> first
                if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
                    println!(">>>>>> Senzing Err (SzBadInputError): {}", sz);
                    sz.for_all();
                } else if let Some(sz) = err.downcast_ref::<SenzingError>() {
                    // Try downcasting to default SenzingError (i.e., SenzingError<SzErrorX>)
                    println!(">>>>>> Senzing Err (default): {}", sz);
                    sz.for_all();
                } else {
                    println!(">>>>>> Non-Senzing")
                }
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type_3() {
        // Helper enum to enable match syntax for different error types
        enum ErrorType<'a> {
            BadInput(&'a SenzingError<SzBadInputError>),
            License(&'a SenzingError<SzLicenseError>),
            RetryTimeout(&'a SenzingError<SzRetryTimeoutExceededError>),
            Other,
        }

        let result = mock_senzing_function_error();
        match result {
            Ok(string) => {
                println!(">>>>>> {}", string);
            }
            Err(err) => {
                println!(">>>>>> In error: {:?}", err);

                // Determine which error type we have
                let error_type =
                    if let Some(sz) = err.downcast_ref::<SenzingError<SzBadInputError>>() {
                        ErrorType::BadInput(sz)
                    } else if let Some(sz) = err.downcast_ref::<SenzingError<SzLicenseError>>() {
                        ErrorType::License(sz)
                    } else if let Some(sz) =
                        err.downcast_ref::<SenzingError<SzRetryTimeoutExceededError>>()
                    {
                        ErrorType::RetryTimeout(sz)
                    } else {
                        ErrorType::Other
                    };

                // Now we can use match to differentiate
                match error_type {
                    ErrorType::BadInput(sz) => {
                        println!(">>>>>> Senzing Err (SzBadInputError): {}", sz);
                    }
                    ErrorType::License(sz) => {
                        println!(">>>>>> Senzing Err (SzLicenseError): {}", sz);
                    }
                    ErrorType::RetryTimeout(sz) => {
                        println!(">>>>>> Senzing Err (SzRetryTimeoutExceededError): {}", sz);
                    }
                    ErrorType::Other => {
                        println!(">>>>>> Non-Senzing or unknown error type");
                    }
                }
            }
        }
    }

    #[test]
    fn test_match_senzing_error_type_4() {
        let result = mock_senzing_function_error();
        match result {
            Ok(string) => {
                println!(">>>>>> {}", string);
            }
            Err(err) => {
                println!(">>>>>> In error: {:?}", err);

                if let Some(senzing_error) = err.downcast_ref::<SenzingError>() {
                    let senzing_error_type = senzing_error.error_type();
                } else {
                    println!(">>>>>> Non-Senzing or unknown error type");
                }
            }
        }
    }
}
