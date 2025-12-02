// use serde_json::Value;

pub mod szproduct_y {

    tonic::include_proto!("szproduct");

    use sz_product_client::SzProductClient;
    use tonic::transport::Channel;

    pub struct SzProduct {
        pub grpc_client: SzProductClient<Channel>,
    }

    impl SzProduct {
        pub async fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let request = tonic::Request::new(GetVersionRequest {});
            let response = self.grpc_client.get_version(request).await?;
            Ok(response.get_ref().clone().result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::szproduct_y::sz_product_client::SzProductClient;
    use tonic::transport::Channel;

    fn is_valid_json(s: String) -> bool {
        println!(">>>>>> is_valid_json={:?}", s);
        serde_json::from_str::<serde_json::Value>(&s).is_ok()
    }

    async fn get_grpc_client() -> SzProductClient<Channel> {
        match SzProductClient::connect("http://0.0.0.0:8261").await {
            Ok(client) => client,
            Err(_) => panic!("Cannot get gRPC channel"),
        }
    }

    #[tokio::test]
    async fn test_get_version() {
        let client = get_grpc_client().await;
        let mut szproduct = super::szproduct_y::SzProduct {
            grpc_client: client,
        };

        let result = szproduct.get_version().await;
        assert!(result.is_ok_and(is_valid_json));
    }
}
