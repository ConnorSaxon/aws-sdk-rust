// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeWorkspacesConnectionStatus`](crate::client::fluent_builders::DescribeWorkspacesConnectionStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_ids(Vec<String>)`](crate::client::fluent_builders::DescribeWorkspacesConnectionStatus::workspace_ids) / [`set_workspace_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeWorkspacesConnectionStatus::set_workspace_ids): <p>The identifiers of the WorkSpaces. You can specify up to 25 WorkSpaces.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeWorkspacesConnectionStatus::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeWorkspacesConnectionStatus::set_next_token): <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
                            /// - On success, responds with [`DescribeWorkspacesConnectionStatusOutput`](crate::output::DescribeWorkspacesConnectionStatusOutput) with field(s):
    ///   - [`workspaces_connection_status(Option<Vec<WorkspaceConnectionStatus>>)`](crate::output::DescribeWorkspacesConnectionStatusOutput::workspaces_connection_status): <p>Information about the connection status of the WorkSpace.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeWorkspacesConnectionStatusOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
                            /// - On failure, responds with [`SdkError<DescribeWorkspacesConnectionStatusError>`](crate::error::DescribeWorkspacesConnectionStatusError)
    pub fn describe_workspaces_connection_status(&self) -> crate::client::fluent_builders::DescribeWorkspacesConnectionStatus {
                                crate::client::fluent_builders::DescribeWorkspacesConnectionStatus::new(self.handle.clone())
                            }
}

