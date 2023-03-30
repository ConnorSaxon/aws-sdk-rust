// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWorkspaceAuthentication`](crate::client::fluent_builders::UpdateWorkspaceAuthentication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::set_workspace_id): <p>The ID of the workspace to update the authentication for.</p>
    ///   - [`authentication_providers(Vec<AuthenticationProviderTypes>)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::authentication_providers) / [`set_authentication_providers(Option<Vec<AuthenticationProviderTypes>>)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::set_authentication_providers): <p>Specifies whether this workspace uses SAML 2.0, IAM Identity Center (successor to Single Sign-On), or both to authenticate users for using the Grafana console within a workspace. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/authentication-in-AMG.html">User authentication in Amazon Managed Grafana</a>.</p>
    ///   - [`saml_configuration(SamlConfiguration)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::saml_configuration) / [`set_saml_configuration(Option<SamlConfiguration>)`](crate::client::fluent_builders::UpdateWorkspaceAuthentication::set_saml_configuration): <p>If the workspace uses SAML, use this structure to map SAML assertion attributes to workspace user information and define which groups in the assertion attribute are to have the <code>Admin</code> and <code>Editor</code> roles in the workspace.</p>
                            /// - On success, responds with [`UpdateWorkspaceAuthenticationOutput`](crate::output::UpdateWorkspaceAuthenticationOutput) with field(s):
    ///   - [`authentication(Option<AuthenticationDescription>)`](crate::output::UpdateWorkspaceAuthenticationOutput::authentication): <p>A structure that describes the user authentication for this workspace after the update is made.</p>
                            /// - On failure, responds with [`SdkError<UpdateWorkspaceAuthenticationError>`](crate::error::UpdateWorkspaceAuthenticationError)
    pub fn update_workspace_authentication(&self) -> crate::client::fluent_builders::UpdateWorkspaceAuthentication {
                                crate::client::fluent_builders::UpdateWorkspaceAuthentication::new(self.handle.clone())
                            }
}

