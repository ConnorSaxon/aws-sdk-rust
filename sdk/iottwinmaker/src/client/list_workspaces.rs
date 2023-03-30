// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListWorkspaces`](crate::client::fluent_builders::ListWorkspaces) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListWorkspaces::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListWorkspaces::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListWorkspaces::set_max_results): <p>The maximum number of results to return at one time. The default is 25.</p>  <p>Valid Range: Minimum value of 1. Maximum value of 250.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListWorkspaces::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListWorkspaces::set_next_token): <p>The string that specifies the next page of results.</p>
                            /// - On success, responds with [`ListWorkspacesOutput`](crate::output::ListWorkspacesOutput) with field(s):
    ///   - [`workspace_summaries(Option<Vec<WorkspaceSummary>>)`](crate::output::ListWorkspacesOutput::workspace_summaries): <p>A list of objects that contain information about the workspaces.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListWorkspacesOutput::next_token): <p>The string that specifies the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListWorkspacesError>`](crate::error::ListWorkspacesError)
    pub fn list_workspaces(&self) -> crate::client::fluent_builders::ListWorkspaces {
                                crate::client::fluent_builders::ListWorkspaces::new(self.handle.clone())
                            }
}

