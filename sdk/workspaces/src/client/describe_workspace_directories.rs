// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeWorkspaceDirectories`](crate::client::fluent_builders::DescribeWorkspaceDirectories) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeWorkspaceDirectories::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_ids(Vec<String>)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::directory_ids) / [`set_directory_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::set_directory_ids): <p>The identifiers of the directories. If the value is null, all directories are retrieved.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::set_limit): <p>The maximum number of directories to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeWorkspaceDirectories::set_next_token): <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
                            /// - On success, responds with [`DescribeWorkspaceDirectoriesOutput`](crate::output::DescribeWorkspaceDirectoriesOutput) with field(s):
    ///   - [`directories(Option<Vec<WorkspaceDirectory>>)`](crate::output::DescribeWorkspaceDirectoriesOutput::directories): <p>Information about the directories.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeWorkspaceDirectoriesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
                            /// - On failure, responds with [`SdkError<DescribeWorkspaceDirectoriesError>`](crate::error::DescribeWorkspaceDirectoriesError)
    pub fn describe_workspace_directories(&self) -> crate::client::fluent_builders::DescribeWorkspaceDirectories {
                                crate::client::fluent_builders::DescribeWorkspaceDirectories::new(self.handle.clone())
                            }
}

