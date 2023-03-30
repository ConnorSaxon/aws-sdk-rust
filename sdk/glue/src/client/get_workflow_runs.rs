// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWorkflowRuns`](crate::client::fluent_builders::GetWorkflowRuns) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetWorkflowRuns::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetWorkflowRuns::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetWorkflowRuns::set_name): <p>Name of the workflow whose metadata of runs should be returned.</p>
    ///   - [`include_graph(bool)`](crate::client::fluent_builders::GetWorkflowRuns::include_graph) / [`set_include_graph(Option<bool>)`](crate::client::fluent_builders::GetWorkflowRuns::set_include_graph): <p>Specifies whether to include the workflow graph in response or not.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetWorkflowRuns::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetWorkflowRuns::set_next_token): <p>The maximum size of the response.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetWorkflowRuns::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetWorkflowRuns::set_max_results): <p>The maximum number of workflow runs to be included in the response.</p>
                            /// - On success, responds with [`GetWorkflowRunsOutput`](crate::output::GetWorkflowRunsOutput) with field(s):
    ///   - [`runs(Option<Vec<WorkflowRun>>)`](crate::output::GetWorkflowRunsOutput::runs): <p>A list of workflow run metadata objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetWorkflowRunsOutput::next_token): <p>A continuation token, if not all requested workflow runs have been returned.</p>
                            /// - On failure, responds with [`SdkError<GetWorkflowRunsError>`](crate::error::GetWorkflowRunsError)
    pub fn get_workflow_runs(&self) -> crate::client::fluent_builders::GetWorkflowRuns {
                                crate::client::fluent_builders::GetWorkflowRuns::new(self.handle.clone())
                            }
}

