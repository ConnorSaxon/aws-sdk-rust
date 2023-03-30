// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableOrganizationAdminAccount`](crate::client::fluent_builders::DisableOrganizationAdminAccount) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`admin_account_id(impl Into<String>)`](crate::client::fluent_builders::DisableOrganizationAdminAccount::admin_account_id) / [`set_admin_account_id(Option<String>)`](crate::client::fluent_builders::DisableOrganizationAdminAccount::set_admin_account_id): <p>The Amazon Web Services Account ID for the organizations account to be disabled as a GuardDuty delegated administrator.</p>
                            /// - On success, responds with [`DisableOrganizationAdminAccountOutput`](crate::output::DisableOrganizationAdminAccountOutput)
                            /// - On failure, responds with [`SdkError<DisableOrganizationAdminAccountError>`](crate::error::DisableOrganizationAdminAccountError)
    pub fn disable_organization_admin_account(&self) -> crate::client::fluent_builders::DisableOrganizationAdminAccount {
                                crate::client::fluent_builders::DisableOrganizationAdminAccount::new(self.handle.clone())
                            }
}

