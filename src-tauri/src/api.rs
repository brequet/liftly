#[taurpc::ipc_type]
pub struct HelloParams {
    name: String,
}

#[taurpc::ipc_type]
pub struct HelloResponse {
    message: String,
}

#[taurpc::procedures(export_to = "../src/lib/taurpc/types.ts")]
pub trait Api {
    async fn hello_world();

    async fn hello(params: HelloParams) -> HelloResponse;
}

#[derive(Clone)]
pub struct ApiImpl;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn hello_world(self) {
        println!("Hello world");
    }

    async fn hello(self, params: HelloParams) -> HelloResponse {
        println!("Hello {}", params.name);
        HelloResponse {
            message: format!("Hello {}", params.name),
        }
    }
}
