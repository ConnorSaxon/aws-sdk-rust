// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWorkflow`](crate::client::fluent_builders::GetWorkflow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetWorkflow::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetWorkflow::set_id): <p>The ID of the migration workflow.</p>
                            /// - On success, responds with [`GetWorkflowOutput`](crate::output::GetWorkflowOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::GetWorkflowOutput::id): <p>The ID of the migration workflow.</p>
    ///   - [`arn(Option<String>)`](crate::output::GetWorkflowOutput::arn): <p>The Amazon Resource Name (ARN) of the migration workflow.</p>
    ///   - [`name(Option<String>)`](crate::output::GetWorkflowOutput::name): <p>The name of the migration workflow.</p>
    ///   - [`description(Option<String>)`](crate::output::GetWorkflowOutput::description): <p>The description of the migration workflow.</p>
    ///   - [`template_id(Option<String>)`](crate::output::GetWorkflowOutput::template_id): <p>The ID of the template.</p>
    ///   - [`ads_application_configuration_id(Option<String>)`](crate::output::GetWorkflowOutput::ads_application_configuration_id): <p>The configuration ID of the application configured in Application Discovery Service.</p>
    ///   - [`ads_application_name(Option<String>)`](crate::output::GetWorkflowOutput::ads_application_name): <p>The name of the application configured in Application Discovery Service.</p>
    ///   - [`status(Option<MigrationWorkflowStatusEnum>)`](crate::output::GetWorkflowOutput::status): <p>The status of the migration workflow.</p>
    ///   - [`status_message(Option<String>)`](crate::output::GetWorkflowOutput::status_message): <p>The status message of the migration workflow.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::GetWorkflowOutput::creation_time): <p>The time at which the migration workflow was created.</p>
    ///   - [`last_start_time(Option<DateTime>)`](crate::output::GetWorkflowOutput::last_start_time): <p>The time at which the migration workflow was last started.</p>
    ///   - [`last_stop_time(Option<DateTime>)`](crate::output::GetWorkflowOutput::last_stop_time): <p>The time at which the migration workflow was last stopped.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::GetWorkflowOutput::last_modified_time): <p>The time at which the migration workflow was last modified.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::output::GetWorkflowOutput::end_time): <p>The time at which the migration workflow ended.</p>
    ///   - [`tools(Option<Vec<Tool>>)`](crate::output::GetWorkflowOutput::tools): <p>List of AWS services utilized in a migration workflow.</p>
    ///   - [`total_steps(Option<i32>)`](crate::output::GetWorkflowOutput::total_steps): <p>The total number of steps in the migration workflow.</p>
    ///   - [`completed_steps(Option<i32>)`](crate::output::GetWorkflowOutput::completed_steps): <p>Get a list of completed steps in the migration workflow.</p>
    ///   - [`workflow_inputs(Option<HashMap<String, StepInput>>)`](crate::output::GetWorkflowOutput::workflow_inputs): <p>The inputs required for creating the migration workflow.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetWorkflowOutput::tags): <p>The tags added to the migration workflow.</p>
    ///   - [`workflow_bucket(Option<String>)`](crate::output::GetWorkflowOutput::workflow_bucket): <p>The Amazon S3 bucket where the migration logs are stored.</p>
                            /// - On failure, responds with [`SdkError<GetWorkflowError>`](crate::error::GetWorkflowError)
    pub fn get_workflow(&self) -> crate::client::fluent_builders::GetWorkflow {
                                crate::client::fluent_builders::GetWorkflow::new(self.handle.clone())
                            }
}

