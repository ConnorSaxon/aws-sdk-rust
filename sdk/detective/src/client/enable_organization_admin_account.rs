// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableOrganizationAdminAccount`](crate::client::fluent_builders::EnableOrganizationAdminAccount) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::EnableOrganizationAdminAccount::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::EnableOrganizationAdminAccount::set_account_id): <p>The Amazon Web Services account identifier of the account to designate as the Detective administrator account for the organization.</p>
                            /// - On success, responds with [`EnableOrganizationAdminAccountOutput`](crate::output::EnableOrganizationAdminAccountOutput)
                            /// - On failure, responds with [`SdkError<EnableOrganizationAdminAccountError>`](crate::error::EnableOrganizationAdminAccountError)
    pub fn enable_organization_admin_account(&self) -> crate::client::fluent_builders::EnableOrganizationAdminAccount {
                                crate::client::fluent_builders::EnableOrganizationAdminAccount::new(self.handle.clone())
                            }
}

