// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWorkflowStep`](crate::client::fluent_builders::UpdateWorkflowStep) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_id): <p>The ID of the step.</p>
    ///   - [`step_group_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::step_group_id) / [`set_step_group_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_step_group_id): <p>The ID of the step group.</p>
    ///   - [`workflow_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::workflow_id) / [`set_workflow_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_workflow_id): <p>The ID of the migration workflow.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_name): <p>The name of the step.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_description): <p>The description of the step.</p>
    ///   - [`step_action_type(StepActionType)`](crate::client::fluent_builders::UpdateWorkflowStep::step_action_type) / [`set_step_action_type(Option<StepActionType>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_step_action_type): <p>The action type of the step. You must run and update the status of a manual step for the workflow to continue after the completion of the step.</p>
    ///   - [`workflow_step_automation_configuration(WorkflowStepAutomationConfiguration)`](crate::client::fluent_builders::UpdateWorkflowStep::workflow_step_automation_configuration) / [`set_workflow_step_automation_configuration(Option<WorkflowStepAutomationConfiguration>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_workflow_step_automation_configuration): <p>The custom script to run tests on the source and target environments.</p>
    ///   - [`step_target(Vec<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::step_target) / [`set_step_target(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_step_target): <p>The servers on which a step will be run.</p>
    ///   - [`outputs(Vec<WorkflowStepOutput>)`](crate::client::fluent_builders::UpdateWorkflowStep::outputs) / [`set_outputs(Option<Vec<WorkflowStepOutput>>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_outputs): <p>The outputs of a step.</p>
    ///   - [`previous(Vec<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::previous) / [`set_previous(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_previous): <p>The previous step.</p>
    ///   - [`next(Vec<String>)`](crate::client::fluent_builders::UpdateWorkflowStep::next) / [`set_next(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_next): <p>The next step.</p>
    ///   - [`status(StepStatus)`](crate::client::fluent_builders::UpdateWorkflowStep::status) / [`set_status(Option<StepStatus>)`](crate::client::fluent_builders::UpdateWorkflowStep::set_status): <p>The status of the step.</p>
                            /// - On success, responds with [`UpdateWorkflowStepOutput`](crate::output::UpdateWorkflowStepOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::UpdateWorkflowStepOutput::id): <p>The ID of the step.</p>
    ///   - [`step_group_id(Option<String>)`](crate::output::UpdateWorkflowStepOutput::step_group_id): <p>The ID of the step group.</p>
    ///   - [`workflow_id(Option<String>)`](crate::output::UpdateWorkflowStepOutput::workflow_id): <p>The ID of the migration workflow.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateWorkflowStepOutput::name): <p>The name of the step.</p>
                            /// - On failure, responds with [`SdkError<UpdateWorkflowStepError>`](crate::error::UpdateWorkflowStepError)
    pub fn update_workflow_step(&self) -> crate::client::fluent_builders::UpdateWorkflowStep {
                                crate::client::fluent_builders::UpdateWorkflowStep::new(self.handle.clone())
                            }
}

