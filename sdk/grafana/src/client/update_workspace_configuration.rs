// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWorkspaceConfiguration`](crate::client::fluent_builders::UpdateWorkspaceConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceConfiguration::configuration) / [`set_configuration(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceConfiguration::set_configuration): <p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceConfiguration::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceConfiguration::set_workspace_id): <p>The ID of the workspace to update.</p>
                            /// - On success, responds with [`UpdateWorkspaceConfigurationOutput`](crate::output::UpdateWorkspaceConfigurationOutput)
                            /// - On failure, responds with [`SdkError<UpdateWorkspaceConfigurationError>`](crate::error::UpdateWorkspaceConfigurationError)
    pub fn update_workspace_configuration(&self) -> crate::client::fluent_builders::UpdateWorkspaceConfiguration {
                                crate::client::fluent_builders::UpdateWorkspaceConfiguration::new(self.handle.clone())
                            }
}

