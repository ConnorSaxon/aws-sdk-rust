// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelImportTask`](crate::client::fluent_builders::CancelImportTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cancel_reason(impl Into<String>)`](crate::client::fluent_builders::CancelImportTask::cancel_reason) / [`set_cancel_reason(Option<String>)`](crate::client::fluent_builders::CancelImportTask::set_cancel_reason): <p>The reason for canceling the task.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CancelImportTask::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CancelImportTask::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`import_task_id(impl Into<String>)`](crate::client::fluent_builders::CancelImportTask::import_task_id) / [`set_import_task_id(Option<String>)`](crate::client::fluent_builders::CancelImportTask::set_import_task_id): <p>The ID of the import image or import snapshot task to be canceled.</p>
                            /// - On success, responds with [`CancelImportTaskOutput`](crate::output::CancelImportTaskOutput) with field(s):
    ///   - [`import_task_id(Option<String>)`](crate::output::CancelImportTaskOutput::import_task_id): <p>The ID of the task being canceled.</p>
    ///   - [`previous_state(Option<String>)`](crate::output::CancelImportTaskOutput::previous_state): <p>The current state of the task being canceled.</p>
    ///   - [`state(Option<String>)`](crate::output::CancelImportTaskOutput::state): <p>The current state of the task being canceled.</p>
                            /// - On failure, responds with [`SdkError<CancelImportTaskError>`](crate::error::CancelImportTaskError)
    pub fn cancel_import_task(&self) -> crate::client::fluent_builders::CancelImportTask {
                                crate::client::fluent_builders::CancelImportTask::new(self.handle.clone())
                            }
}

