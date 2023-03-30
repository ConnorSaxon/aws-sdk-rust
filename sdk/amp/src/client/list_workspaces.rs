// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListWorkspaces`](crate::client::fluent_builders::ListWorkspaces) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListWorkspaces::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListWorkspaces::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListWorkspaces::set_next_token): Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
    ///   - [`alias(impl Into<String>)`](crate::client::fluent_builders::ListWorkspaces::alias) / [`set_alias(Option<String>)`](crate::client::fluent_builders::ListWorkspaces::set_alias): Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListWorkspaces::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListWorkspaces::set_max_results): Maximum results to return in response (default=100, maximum=1000).
                            /// - On success, responds with [`ListWorkspacesOutput`](crate::output::ListWorkspacesOutput) with field(s):
    ///   - [`workspaces(Option<Vec<WorkspaceSummary>>)`](crate::output::ListWorkspacesOutput::workspaces): The list of existing workspaces, including those undergoing creation or deletion.
    ///   - [`next_token(Option<String>)`](crate::output::ListWorkspacesOutput::next_token): Pagination token to use when requesting the next page in this list.
                            /// - On failure, responds with [`SdkError<ListWorkspacesError>`](crate::error::ListWorkspacesError)
    pub fn list_workspaces(&self) -> crate::client::fluent_builders::ListWorkspaces {
                                crate::client::fluent_builders::ListWorkspaces::new(self.handle.clone())
                            }
}

