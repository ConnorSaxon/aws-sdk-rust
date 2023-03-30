// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListProvisioningTemplateVersions`](crate::client::fluent_builders::ListProvisioningTemplateVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListProvisioningTemplateVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::set_template_name): <p>The name of the provisioning template.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::set_max_results): <p>The maximum number of results to return at one time.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListProvisioningTemplateVersions::set_next_token): <p>A token to retrieve the next set of results.</p>
                            /// - On success, responds with [`ListProvisioningTemplateVersionsOutput`](crate::output::ListProvisioningTemplateVersionsOutput) with field(s):
    ///   - [`versions(Option<Vec<ProvisioningTemplateVersionSummary>>)`](crate::output::ListProvisioningTemplateVersionsOutput::versions): <p>The list of provisioning template versions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListProvisioningTemplateVersionsOutput::next_token): <p>A token to retrieve the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListProvisioningTemplateVersionsError>`](crate::error::ListProvisioningTemplateVersionsError)
    pub fn list_provisioning_template_versions(&self) -> crate::client::fluent_builders::ListProvisioningTemplateVersions {
                                crate::client::fluent_builders::ListProvisioningTemplateVersions::new(self.handle.clone())
                            }
}

