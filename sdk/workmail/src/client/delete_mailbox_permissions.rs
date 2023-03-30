// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMailboxPermissions`](crate::client::fluent_builders::DeleteMailboxPermissions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::set_organization_id): <p>The identifier of the organization under which the member (user or group) exists.</p>
    ///   - [`entity_id(impl Into<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::entity_id) / [`set_entity_id(Option<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::set_entity_id): <p>The identifier of the member (user or group) that owns the mailbox.</p>
    ///   - [`grantee_id(impl Into<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::grantee_id) / [`set_grantee_id(Option<String>)`](crate::client::fluent_builders::DeleteMailboxPermissions::set_grantee_id): <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
                            /// - On success, responds with [`DeleteMailboxPermissionsOutput`](crate::output::DeleteMailboxPermissionsOutput)
                            /// - On failure, responds with [`SdkError<DeleteMailboxPermissionsError>`](crate::error::DeleteMailboxPermissionsError)
    pub fn delete_mailbox_permissions(&self) -> crate::client::fluent_builders::DeleteMailboxPermissions {
                                crate::client::fluent_builders::DeleteMailboxPermissions::new(self.handle.clone())
                            }
}

