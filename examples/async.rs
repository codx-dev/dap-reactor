use std::env;

use dap_reactor::prelude::*;
use tracing_subscriber::filter::EnvFilter;

struct Service {
    _events: mpsc::Sender<Event>,
    _requests: mpsc::Sender<ReactorReverseRequest>,
}

#[async_trait::async_trait]
impl Backend for Service {
    async fn init(
        events: mpsc::Sender<Event>,
        requests: mpsc::Sender<ReactorReverseRequest>,
    ) -> Self {
        let (_events, _requests) = (events, requests);

        Service { _events, _requests }
    }

    async fn request(&mut self, request: Request) -> Option<Response> {
        match request {
            Request::Attach { arguments: _ } => Some(Response::Attach),
            Request::Terminate { arguments: _ } => Some(Response::Terminate),
            Request::Disconnect { arguments: _ } => Some(Response::Error {
                command: "not implemented".into(),
                error: ProtocolResponseError {
                    message: None,
                    body: None,
                },
            }),
            _ => None,
        }
    }

    async fn response(&mut self, id: u64, response: Response) {
        println!("got a response {} from the client: {:?}", id, response);
    }
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

    Reactor::<Service>::new()
        .with_capacity(50)
        .bind("127.0.0.1:5647")
        .await
        .expect("failed to run service");
}
