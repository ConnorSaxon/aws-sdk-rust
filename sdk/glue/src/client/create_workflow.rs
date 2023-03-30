// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateWorkflow`](crate::client::fluent_builders::CreateWorkflow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateWorkflow::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateWorkflow::set_name): <p>The name to be assigned to the workflow. It should be unique within your account.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateWorkflow::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateWorkflow::set_description): <p>A description of the workflow.</p>
    ///   - [`default_run_properties(HashMap<String, String>)`](crate::client::fluent_builders::CreateWorkflow::default_run_properties) / [`set_default_run_properties(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateWorkflow::set_default_run_properties): <p>A collection of properties to be used as part of each execution of the workflow.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateWorkflow::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateWorkflow::set_tags): <p>The tags to be used with this workflow.</p>
    ///   - [`max_concurrent_runs(i32)`](crate::client::fluent_builders::CreateWorkflow::max_concurrent_runs) / [`set_max_concurrent_runs(Option<i32>)`](crate::client::fluent_builders::CreateWorkflow::set_max_concurrent_runs): <p>You can use this parameter to prevent unwanted multiple updates to data, to control costs, or in some cases, to prevent exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.</p>
                            /// - On success, responds with [`CreateWorkflowOutput`](crate::output::CreateWorkflowOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateWorkflowOutput::name): <p>The name of the workflow which was provided as part of the request.</p>
                            /// - On failure, responds with [`SdkError<CreateWorkflowError>`](crate::error::CreateWorkflowError)
    pub fn create_workflow(&self) -> crate::client::fluent_builders::CreateWorkflow {
                                crate::client::fluent_builders::CreateWorkflow::new(self.handle.clone())
                            }
}

