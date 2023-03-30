// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteProvisioningTemplateVersion`](crate::client::fluent_builders::DeleteProvisioningTemplateVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::DeleteProvisioningTemplateVersion::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::DeleteProvisioningTemplateVersion::set_template_name): <p>The name of the provisioning template version to delete.</p>
    ///   - [`version_id(i32)`](crate::client::fluent_builders::DeleteProvisioningTemplateVersion::version_id) / [`set_version_id(Option<i32>)`](crate::client::fluent_builders::DeleteProvisioningTemplateVersion::set_version_id): <p>The provisioning template version ID to delete.</p>
                            /// - On success, responds with [`DeleteProvisioningTemplateVersionOutput`](crate::output::DeleteProvisioningTemplateVersionOutput)
                            /// - On failure, responds with [`SdkError<DeleteProvisioningTemplateVersionError>`](crate::error::DeleteProvisioningTemplateVersionError)
    pub fn delete_provisioning_template_version(&self) -> crate::client::fluent_builders::DeleteProvisioningTemplateVersion {
                                crate::client::fluent_builders::DeleteProvisioningTemplateVersion::new(self.handle.clone())
                            }
}

