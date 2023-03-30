// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWorkspaceAlias`](crate::client::fluent_builders::UpdateWorkspaceAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::set_workspace_id): The ID of the workspace being updated.
    ///   - [`alias(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::alias) / [`set_alias(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::set_alias): The new alias of the workspace.
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateWorkspaceAlias::set_client_token): Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
                            /// - On success, responds with [`UpdateWorkspaceAliasOutput`](crate::output::UpdateWorkspaceAliasOutput)
                            /// - On failure, responds with [`SdkError<UpdateWorkspaceAliasError>`](crate::error::UpdateWorkspaceAliasError)
    pub fn update_workspace_alias(&self) -> crate::client::fluent_builders::UpdateWorkspaceAlias {
                                crate::client::fluent_builders::UpdateWorkspaceAlias::new(self.handle.clone())
                            }
}

