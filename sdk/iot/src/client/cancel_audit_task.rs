// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelAuditTask`](crate::client::fluent_builders::CancelAuditTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`task_id(impl Into<String>)`](crate::client::fluent_builders::CancelAuditTask::task_id) / [`set_task_id(Option<String>)`](crate::client::fluent_builders::CancelAuditTask::set_task_id): <p>The ID of the audit you want to cancel. You can only cancel an audit that is "IN_PROGRESS".</p>
                            /// - On success, responds with [`CancelAuditTaskOutput`](crate::output::CancelAuditTaskOutput)
                            /// - On failure, responds with [`SdkError<CancelAuditTaskError>`](crate::error::CancelAuditTaskError)
    pub fn cancel_audit_task(&self) -> crate::client::fluent_builders::CancelAuditTask {
                                crate::client::fluent_builders::CancelAuditTask::new(self.handle.clone())
                            }
}

