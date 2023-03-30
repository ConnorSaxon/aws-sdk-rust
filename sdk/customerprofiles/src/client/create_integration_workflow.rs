// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateIntegrationWorkflow`](crate::client::fluent_builders::CreateIntegrationWorkflow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_domain_name): <p>The unique name of the domain.</p>
    ///   - [`workflow_type(WorkflowType)`](crate::client::fluent_builders::CreateIntegrationWorkflow::workflow_type) / [`set_workflow_type(Option<WorkflowType>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_workflow_type): <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    ///   - [`integration_config(IntegrationConfig)`](crate::client::fluent_builders::CreateIntegrationWorkflow::integration_config) / [`set_integration_config(Option<IntegrationConfig>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_integration_config): <p>Configuration data for integration workflow.</p>
    ///   - [`object_type_name(impl Into<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::object_type_name) / [`set_object_type_name(Option<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_object_type_name): <p>The name of the profile object type.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role. Customer Profiles assumes this role to create resources on your behalf as part of workflow execution.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateIntegrationWorkflow::set_tags): <p>The tags used to organize, track, or control access for this resource.</p>
                            /// - On success, responds with [`CreateIntegrationWorkflowOutput`](crate::output::CreateIntegrationWorkflowOutput) with field(s):
    ///   - [`workflow_id(Option<String>)`](crate::output::CreateIntegrationWorkflowOutput::workflow_id): <p>Unique identifier for the workflow.</p>
    ///   - [`message(Option<String>)`](crate::output::CreateIntegrationWorkflowOutput::message): <p>A message indicating create request was received.</p>
                            /// - On failure, responds with [`SdkError<CreateIntegrationWorkflowError>`](crate::error::CreateIntegrationWorkflowError)
    pub fn create_integration_workflow(&self) -> crate::client::fluent_builders::CreateIntegrationWorkflow {
                                crate::client::fluent_builders::CreateIntegrationWorkflow::new(self.handle.clone())
                            }
}

