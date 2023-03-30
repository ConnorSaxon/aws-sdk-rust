// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RecordHandlerProgress`](crate::client::fluent_builders::RecordHandlerProgress) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bearer_token(impl Into<String>)`](crate::client::fluent_builders::RecordHandlerProgress::bearer_token) / [`set_bearer_token(Option<String>)`](crate::client::fluent_builders::RecordHandlerProgress::set_bearer_token): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`operation_status(OperationStatus)`](crate::client::fluent_builders::RecordHandlerProgress::operation_status) / [`set_operation_status(Option<OperationStatus>)`](crate::client::fluent_builders::RecordHandlerProgress::set_operation_status): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`current_operation_status(OperationStatus)`](crate::client::fluent_builders::RecordHandlerProgress::current_operation_status) / [`set_current_operation_status(Option<OperationStatus>)`](crate::client::fluent_builders::RecordHandlerProgress::set_current_operation_status): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`status_message(impl Into<String>)`](crate::client::fluent_builders::RecordHandlerProgress::status_message) / [`set_status_message(Option<String>)`](crate::client::fluent_builders::RecordHandlerProgress::set_status_message): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`error_code(HandlerErrorCode)`](crate::client::fluent_builders::RecordHandlerProgress::error_code) / [`set_error_code(Option<HandlerErrorCode>)`](crate::client::fluent_builders::RecordHandlerProgress::set_error_code): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`resource_model(impl Into<String>)`](crate::client::fluent_builders::RecordHandlerProgress::resource_model) / [`set_resource_model(Option<String>)`](crate::client::fluent_builders::RecordHandlerProgress::set_resource_model): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::RecordHandlerProgress::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::RecordHandlerProgress::set_client_request_token): <p>Reserved for use by the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/what-is-cloudformation-cli.html">CloudFormation CLI</a>.</p>
                            /// - On success, responds with [`RecordHandlerProgressOutput`](crate::output::RecordHandlerProgressOutput)
                            /// - On failure, responds with [`SdkError<RecordHandlerProgressError>`](crate::error::RecordHandlerProgressError)
    pub fn record_handler_progress(&self) -> crate::client::fluent_builders::RecordHandlerProgress {
                                crate::client::fluent_builders::RecordHandlerProgress::new(self.handle.clone())
                            }
}

