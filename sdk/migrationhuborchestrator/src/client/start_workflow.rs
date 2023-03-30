// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartWorkflow`](crate::client::fluent_builders::StartWorkflow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::StartWorkflow::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::StartWorkflow::set_id): <p>The ID of the migration workflow.</p>
                            /// - On success, responds with [`StartWorkflowOutput`](crate::output::StartWorkflowOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::StartWorkflowOutput::id): <p>The ID of the migration workflow.</p>
    ///   - [`arn(Option<String>)`](crate::output::StartWorkflowOutput::arn): <p>The Amazon Resource Name (ARN) of the migration workflow.</p>
    ///   - [`status(Option<MigrationWorkflowStatusEnum>)`](crate::output::StartWorkflowOutput::status): <p>The status of the migration workflow.</p>
    ///   - [`status_message(Option<String>)`](crate::output::StartWorkflowOutput::status_message): <p>The status message of the migration workflow.</p>
    ///   - [`last_start_time(Option<DateTime>)`](crate::output::StartWorkflowOutput::last_start_time): <p>The time at which the migration workflow was last started.</p>
                            /// - On failure, responds with [`SdkError<StartWorkflowError>`](crate::error::StartWorkflowError)
    pub fn start_workflow(&self) -> crate::client::fluent_builders::StartWorkflow {
                                crate::client::fluent_builders::StartWorkflow::new(self.handle.clone())
                            }
}

