// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelTaskExecution`](crate::client::fluent_builders::CancelTaskExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`task_execution_arn(impl Into<String>)`](crate::client::fluent_builders::CancelTaskExecution::task_execution_arn) / [`set_task_execution_arn(Option<String>)`](crate::client::fluent_builders::CancelTaskExecution::set_task_execution_arn): <p>The Amazon Resource Name (ARN) of the task execution to stop.</p>
                            /// - On success, responds with [`CancelTaskExecutionOutput`](crate::output::CancelTaskExecutionOutput)
                            /// - On failure, responds with [`SdkError<CancelTaskExecutionError>`](crate::error::CancelTaskExecutionError)
    pub fn cancel_task_execution(&self) -> crate::client::fluent_builders::CancelTaskExecution {
                                crate::client::fluent_builders::CancelTaskExecution::new(self.handle.clone())
                            }
}

