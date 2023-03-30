// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListWorkflows`](crate::client::fluent_builders::ListWorkflows) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListWorkflows::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListWorkflows::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListWorkflows::set_max_results): <p>Specifies the maximum number of workflows to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListWorkflows::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListWorkflows::set_next_token): <p> <code>ListWorkflows</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional workflows.</p>
                            /// - On success, responds with [`ListWorkflowsOutput`](crate::output::ListWorkflowsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListWorkflowsOutput::next_token): <p> <code>ListWorkflows</code> returns the <code>NextToken</code> parameter in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional workflows.</p>
    ///   - [`workflows(Option<Vec<ListedWorkflow>>)`](crate::output::ListWorkflowsOutput::workflows): <p>Returns the <code>Arn</code>, <code>WorkflowId</code>, and <code>Description</code> for each workflow.</p>
                            /// - On failure, responds with [`SdkError<ListWorkflowsError>`](crate::error::ListWorkflowsError)
    pub fn list_workflows(&self) -> crate::client::fluent_builders::ListWorkflows {
                                crate::client::fluent_builders::ListWorkflows::new(self.handle.clone())
                            }
}

