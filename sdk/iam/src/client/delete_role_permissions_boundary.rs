// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRolePermissionsBoundary`](crate::client::fluent_builders::DeleteRolePermissionsBoundary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRolePermissionsBoundary::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::DeleteRolePermissionsBoundary::set_role_name): <p>The name (friendly name, not ARN) of the IAM role from which you want to remove the permissions boundary.</p>
                            /// - On success, responds with [`DeleteRolePermissionsBoundaryOutput`](crate::output::DeleteRolePermissionsBoundaryOutput)
                            /// - On failure, responds with [`SdkError<DeleteRolePermissionsBoundaryError>`](crate::error::DeleteRolePermissionsBoundaryError)
    pub fn delete_role_permissions_boundary(&self) -> crate::client::fluent_builders::DeleteRolePermissionsBoundary {
                                crate::client::fluent_builders::DeleteRolePermissionsBoundary::new(self.handle.clone())
                            }
}

