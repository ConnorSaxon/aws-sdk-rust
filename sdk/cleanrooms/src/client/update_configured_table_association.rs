// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConfiguredTableAssociation`](crate::client::fluent_builders::UpdateConfiguredTableAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configured_table_association_identifier(impl Into<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::configured_table_association_identifier) / [`set_configured_table_association_identifier(Option<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::set_configured_table_association_identifier): <p>The unique identifier for the configured table association to update. Currently accepts the configured table association ID.</p>
    ///   - [`membership_identifier(impl Into<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::set_membership_identifier): <p>The unique ID for the membership that the configured table association belongs to.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::set_description): <p>A new description for the configured table association.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::UpdateConfiguredTableAssociation::set_role_arn): <p>The service will assume this role to access catalog metadata and query the table.</p>
                            /// - On success, responds with [`UpdateConfiguredTableAssociationOutput`](crate::output::UpdateConfiguredTableAssociationOutput) with field(s):
    ///   - [`configured_table_association(Option<ConfiguredTableAssociation>)`](crate::output::UpdateConfiguredTableAssociationOutput::configured_table_association): <p>The entire updated configured table association.</p>
                            /// - On failure, responds with [`SdkError<UpdateConfiguredTableAssociationError>`](crate::error::UpdateConfiguredTableAssociationError)
    pub fn update_configured_table_association(&self) -> crate::client::fluent_builders::UpdateConfiguredTableAssociation {
                                crate::client::fluent_builders::UpdateConfiguredTableAssociation::new(self.handle.clone())
                            }
}

