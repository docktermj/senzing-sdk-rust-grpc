use crate::szdiagnostic::szdiagnostic_y::sz_diagnostic_client::SzDiagnosticClient;
use szdiagnostic_y::CheckRepositoryPerformanceRequest;

pub mod szdiagnostic_y {
    tonic::include_proto!("szdiagnostic");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[tokio::main]
pub async fn check_repository_performance() -> Result<String, Box<dyn std::error::Error>> {
    let mut client = SzDiagnosticClient::connect("http://0.0.0.0:8261").await?;
    let request = tonic::Request::new(CheckRepositoryPerformanceRequest { seconds_to_run: 5 });
    let response = client.check_repository_performance(request).await?;
    let response = response.get_ref().clone();
    Ok(response.result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
