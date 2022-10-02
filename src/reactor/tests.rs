use std::io;

use crate::prelude::*;

struct Service;

impl Service {
    pub fn capabilities() -> Capabilities {
        Capabilities {
            supports_configuration_done_request: true,
            supports_function_breakpoints: true,
            supports_conditional_breakpoints: true,
            supports_hit_conditional_breakpoints: true,
            supports_evaluate_for_hovers: true,
            exception_breakpoint_filters: vec![],
            supports_step_back: true,
            supports_set_variable: false,
            supports_restart_frame: false,
            supports_goto_targets_request: false,
            supports_step_in_targets_request: false,
            supports_completions_request: false,
            completion_trigger_characters: vec![],
            supports_modules_request: false,
            additional_module_columns: vec![],
            supported_checksum_algorithms: vec![
                ChecksumAlgorithm::Md5,
                ChecksumAlgorithm::Sha1,
                ChecksumAlgorithm::Sha256,
                ChecksumAlgorithm::Timestamp,
            ],
            supports_restart_request: true,
            supports_exception_options: false,
            supports_value_formatting_options: false,
            supports_exception_info_request: false,
            support_terminate_debuggee: true,
            support_suspend_debuggee: true,
            supports_delayed_stack_trace_loading: false,
            supports_loaded_sources_request: true,
            supports_log_points: true,
            supports_terminate_threads_request: true,
            supports_set_expression: false,
            supports_terminate_request: true,
            supports_data_breakpoints: true,
            supports_read_memory_request: false,
            supports_write_memory_request: false,
            supports_disassemble_request: false,
            supports_cancel_request: false,
            supports_breakpoint_locations_request: true,
            supports_clipboard_context: false,
            supports_stepping_granularity: false,
            supports_instruction_breakpoints: false,
            supports_exception_filter_options: false,
            supports_single_thread_execution_requests: true,
        }
    }
}

#[async_trait]
impl Backend for Service {
    async fn init(_events: Sender<Event>, _requests: Sender<ReactorReverseRequest>) -> Self {
        Self
    }

    async fn request(&mut self, request: Request) -> Option<Response> {
        match request {
            Request::Initialize {
                arguments: InitializeArguments { .. },
            } => Some(Response::Initialize {
                body: Self::capabilities(),
            }),
            _ => None,
        }
    }

    async fn response(&mut self, _id: u64, _response: Response) {}
}

#[tokio::test]
async fn initialize_works() -> io::Result<()> {
    let reactor = Reactor::<Service>::new()
        .with_capacity(50)
        .bind("127.0.0.1:0")
        .await?;

    let socket = reactor.local_addr()?;

    tokio::spawn(async move {
        reactor.listen().await.ok();
    });

    let mut client = ClientBuilder::new().connect(socket).await?;

    client
        .requests
        .send(ClientRequest {
            seq: None,
            request: Request::Initialize {
                arguments: InitializeArguments {
                    client_id: None,
                    client_name: None,
                    adapter_id: "foo".into(),
                    locale: None,
                    lines_start_at_1: true,
                    column_start_at_1: true,
                    path_format: None,
                    supports_variable_type: true,
                    supports_variable_paging: false,
                    supports_run_in_terminal_request: false,
                    supports_memory_references: false,
                    supports_progress_reporting: false,
                    supports_invalidated_event: false,
                    supports_memory_event: false,
                    supports_args_can_be_interpreted_by_shell: false,
                },
            },
        })
        .await
        .expect("failed to submit request");

    let re = client
        .responses
        .recv()
        .await
        .expect("a response was expected");

    let capabilities = match re.response {
        Response::Initialize { body } => Ok(body),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "wrong response variant",
        )),
    }?;

    assert_eq!(Service::capabilities(), capabilities);

    Ok(())
}
