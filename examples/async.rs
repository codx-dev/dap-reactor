use std::env;

use dap_reactor::prelude::*;
use tracing_subscriber::filter::EnvFilter;

struct Service;

#[async_trait::async_trait]
impl Backend for Service {
    async fn init(_events: mpsc::Sender<Event>, _requests: mpsc::Sender<ReverseRequest>) -> Self {
        Service
    }

    async fn request(&mut self, request: Request) -> Option<Response> {
        match request {
            Request::Attach { arguments: _ } => Some(Response::Attach),
            Request::Disconnect { arguments: _ } => Some(Response::Disconnect),
            Request::Terminate { arguments: _ } => Some(Response::Terminate),
            _ => Some(Response::Error {
                command: "not implemented".into(),
                error: ProtocolResponseError {
                    message: None,
                    body: None,
                },
            }),
        }
    }

    async fn response(&mut self, _response: Response) {}
}

#[tokio::main]
async fn main() {
    let filter = match env::var_os("RUST_LOG") {
        Some(_) => EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided"),
        None => EnvFilter::new("info"),
    };

    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_env_filter(filter)
        .init();

    Reactor::new()
        .bind::<Service, _>("127.0.0.1:5647")
        .await
        .expect("failed to run service");
}
