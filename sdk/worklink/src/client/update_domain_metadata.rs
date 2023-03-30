// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateDomainMetadata`](crate::client::fluent_builders::UpdateDomainMetadata) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::set_fleet_arn): <p>The ARN of the fleet.</p>
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::set_domain_name): <p>The name of the domain.</p>
    ///   - [`display_name(impl Into<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::display_name) / [`set_display_name(Option<String>)`](crate::client::fluent_builders::UpdateDomainMetadata::set_display_name): <p>The name to display.</p>
                            /// - On success, responds with [`UpdateDomainMetadataOutput`](crate::output::UpdateDomainMetadataOutput)
                            /// - On failure, responds with [`SdkError<UpdateDomainMetadataError>`](crate::error::UpdateDomainMetadataError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn update_domain_metadata(&self) -> crate::client::fluent_builders::UpdateDomainMetadata {
                                crate::client::fluent_builders::UpdateDomainMetadata::new(self.handle.clone())
                            }
}

