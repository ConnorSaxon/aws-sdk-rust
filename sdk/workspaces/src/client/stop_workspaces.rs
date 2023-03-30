// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopWorkspaces`](crate::client::fluent_builders::StopWorkspaces) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stop_workspace_requests(Vec<StopRequest>)`](crate::client::fluent_builders::StopWorkspaces::stop_workspace_requests) / [`set_stop_workspace_requests(Option<Vec<StopRequest>>)`](crate::client::fluent_builders::StopWorkspaces::set_stop_workspace_requests): <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
                            /// - On success, responds with [`StopWorkspacesOutput`](crate::output::StopWorkspacesOutput) with field(s):
    ///   - [`failed_requests(Option<Vec<FailedWorkspaceChangeRequest>>)`](crate::output::StopWorkspacesOutput::failed_requests): <p>Information about the WorkSpaces that could not be stopped.</p>
                            /// - On failure, responds with [`SdkError<StopWorkspacesError>`](crate::error::StopWorkspacesError)
    pub fn stop_workspaces(&self) -> crate::client::fluent_builders::StopWorkspaces {
                                crate::client::fluent_builders::StopWorkspaces::new(self.handle.clone())
                            }
}

