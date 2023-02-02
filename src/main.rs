mod configuration;
mod schema;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};
use axum_extra::protobuf::ProtoBuf;

async fn ok() -> &'static str {
    "okay"
}

async fn greet(ProtoBuf(request): ProtoBuf<schema::HelloRequest>) -> impl IntoResponse {
    ProtoBuf(schema::HelloReply {
        message: format!("Hello {}", request.name),
    })
}

#[tokio::main]
async fn main() {
    let configuration = configuration::from_env()
        .expect("unable to initiate configuration");

    let app = Router::new()
        .route("/", get(ok))
        .route("/greet", post(greet));

    Server::bind(&configuration.server_address)
        .serve(app.into_make_service())
        .await
        .expect("unable to bind server")
}
