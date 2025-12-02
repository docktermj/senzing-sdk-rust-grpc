// use serde_json::Value;

pub mod szproduct_y {

    tonic::include_proto!("szproduct");

    use sz_product_client::SzProductClient;
    use tonic::transport::Channel;

    pub struct SzProduct {
        pub grpc_client: SzProductClient<Channel>,
    }

    impl SzProduct {
        pub fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let handle = std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async move {
                    let request = tonic::Request::new(GetVersionRequest {});
                    client.get_version(request).await
                })
            });

            let response = handle.join().unwrap()?;
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

    async fn get_grpc_client_async()
    -> Result<SzProductClient<Channel>, Box<dyn std::error::Error + Send>> {
        // match SzProductClient::connect("http://0.0.0.0:8261").await {
        //     Ok(client) => client,
        //     Err(_) => panic!("Cannot get gRPC channel"),
        // }
        let result = SzProductClient::connect("http://0.0.0.0:8261").await;
        Ok(result.unwrap())
    }

    pub fn get_grpc_client() -> SzProductClient<Channel> {
        // Spawn a new OS thread to avoid runtime conflicts
        let handle = std::thread::spawn(move || {
            // Create a new tokio runtime in the isolated thread
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            // Run the async function on this runtime
            rt.block_on(async move { get_grpc_client_async().await })
        });

        // Wait for the thread to complete and return the result
        let bob = handle.join().unwrap();
        bob.unwrap()
    }

    // fn get_grpc_client() -> SzProductClient<Channel> {
    //     let thingee = get_grpc_client_async();

    //     let bob = thingee.
    //     match thingee {
    //         Ok(xyzzy) => xyzzy,
    //     }

    //     let handle = std::thread::spawn(move || {
    //         let rt = tokio::runtime::Builder::new_current_thread()
    //             .enable_all()
    //             .build()
    //             .unwrap();
    //         rt.block_on(async move { SzProductClient::connect("http://0.0.0.0:8261").await })
    //     });

    //     match handle.join().unwrap() {
    //         Ok(client) => client,
    //         Err(_) => panic!("Cannot get gRPC channel"),
    //     }
    // }

    // fn get_grpc_client() -> SzProductClient<Channel> {
    //     let thingee = get_grpc_client_async();

    //     let bob = thingee.
    //     match thingee {
    //         Ok(xyzzy) => xyzzy,
    //     }

    //     let handle = std::thread::spawn(move || {
    //         let rt = tokio::runtime::Builder::new_current_thread()
    //             .enable_all()
    //             .build()
    //             .unwrap();
    //         rt.block_on(async move { SzProductClient::connect("http://0.0.0.0:8261").await })
    //     });

    //     match handle.join().unwrap() {
    //         Ok(client) => client,
    //         Err(_) => panic!("Cannot get gRPC channel"),
    //     }
    // }

    #[test]
    fn test_get_version() {
        let client = get_grpc_client();
        let mut szproduct = super::szproduct_y::SzProduct {
            grpc_client: client,
        };

        let result = szproduct.get_version();
        println!(">>>>>> result={:?}", result);

        assert!(result.is_ok_and(is_valid_json));
    }
}
