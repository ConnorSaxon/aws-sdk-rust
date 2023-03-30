// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelTask`](crate::client::fluent_builders::CancelTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`task_id(impl Into<String>)`](crate::client::fluent_builders::CancelTask::task_id) / [`set_task_id(Option<String>)`](crate::client::fluent_builders::CancelTask::set_task_id): <p>The ID of the task that you are attempting to cancel. You can retrieve a task ID by using the <code>ListTasks</code> operation.</p>
                            /// - On success, responds with [`CancelTaskOutput`](crate::output::CancelTaskOutput) with field(s):
    ///   - [`task_id(Option<String>)`](crate::output::CancelTaskOutput::task_id): <p>The ID of the task that you are attempting to cancel.</p>
                            /// - On failure, responds with [`SdkError<CancelTaskError>`](crate::error::CancelTaskError)
    pub fn cancel_task(&self) -> crate::client::fluent_builders::CancelTask {
                                crate::client::fluent_builders::CancelTask::new(self.handle.clone())
                            }
}

