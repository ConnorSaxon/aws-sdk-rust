// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterOrganizationDelegatedAdmin`](crate::client::fluent_builders::DeregisterOrganizationDelegatedAdmin) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`delegated_admin_account_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterOrganizationDelegatedAdmin::delegated_admin_account_id) / [`set_delegated_admin_account_id(Option<String>)`](crate::client::fluent_builders::DeregisterOrganizationDelegatedAdmin::set_delegated_admin_account_id): <p>A delegated administrator account ID. This is a member account in an organization that is currently designated as a delegated administrator.</p>
                            /// - On success, responds with [`DeregisterOrganizationDelegatedAdminOutput`](crate::output::DeregisterOrganizationDelegatedAdminOutput)
                            /// - On failure, responds with [`SdkError<DeregisterOrganizationDelegatedAdminError>`](crate::error::DeregisterOrganizationDelegatedAdminError)
    pub fn deregister_organization_delegated_admin(&self) -> crate::client::fluent_builders::DeregisterOrganizationDelegatedAdmin {
                                crate::client::fluent_builders::DeregisterOrganizationDelegatedAdmin::new(self.handle.clone())
                            }
}

