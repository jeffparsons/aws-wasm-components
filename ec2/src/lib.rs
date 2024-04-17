#[allow(warnings)]
mod bindings;

use aws_config::BehaviorVersion;
use bindings::exports::jeffparsons::ec2::ec2::Guest;

struct Component;

impl Guest for Component {
    fn hello_world() -> String {
        dbg!(std::time::SystemTime::now());

        let wasi_http_client = aws_smithy_wasm::wasi::WasiHttpClientBuilder::new().build();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap();

        rt.block_on(async {
            let config = aws_config::defaults(BehaviorVersion::latest())
                .http_client(wasi_http_client)
                .load()
                .await;
            let client = aws_sdk_ec2::Client::new(&config);
            // NOTE: This does not handle pagination.
            let instances = client.describe_instances().send().await.unwrap();
            let instances: Vec<_> = instances
                .reservations()
                .iter()
                .flat_map(|reservation| {
                    reservation
                        .instances()
                        .iter()
                        .map(|instance| instance.instance_id.clone().unwrap())
                })
                .collect();
            dbg!(instances);
        });

        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
