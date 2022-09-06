use std::env;

use dap_reactor::prelude::*;
use tracing_subscriber::filter::EnvFilter;

struct Service;

impl Backend for Service {
    fn request(&mut self, request: Request) -> Response {
        match request {
            Request::Attach { arguments: _ } => Response::Attach,
            Request::Disconnect { arguments: _ } => Response::Disconnect,
            Request::Terminate { arguments: _ } => Response::Terminate,
            _ => Response::Error {
                command: "not implemented".into(),
                error: ProtocolResponseError {
                    message: None,
                    body: None,
                },
            },
        }
    }

    fn event(&mut self, _event: Event) {}
}

#[async_trait::async_trait]
impl AsyncBackend for Service {
    async fn request(&mut self, request: Request) -> Response {
        <Self as Backend>::request(self, request)
    }

    async fn event(&mut self, event: Event) {
        <Self as Backend>::event(self, event);
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

    Reactor::new(Service)
        .bind_async("127.0.0.1:5647")
        .await
        .expect("failed to run service");
}
