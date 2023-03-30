// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopPipelineExecution`](crate::client::fluent_builders::StopPipelineExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pipeline_execution_arn(impl Into<String>)`](crate::client::fluent_builders::StopPipelineExecution::pipeline_execution_arn) / [`set_pipeline_execution_arn(Option<String>)`](crate::client::fluent_builders::StopPipelineExecution::set_pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::StopPipelineExecution::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::StopPipelineExecution::set_client_request_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than once.</p>
                            /// - On success, responds with [`StopPipelineExecutionOutput`](crate::output::StopPipelineExecutionOutput) with field(s):
    ///   - [`pipeline_execution_arn(Option<String>)`](crate::output::StopPipelineExecutionOutput::pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
                            /// - On failure, responds with [`SdkError<StopPipelineExecutionError>`](crate::error::StopPipelineExecutionError)
    pub fn stop_pipeline_execution(&self) -> crate::client::fluent_builders::StopPipelineExecution {
                                crate::client::fluent_builders::StopPipelineExecution::new(self.handle.clone())
                            }
}

