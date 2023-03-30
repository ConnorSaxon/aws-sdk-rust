// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetApiAssociation`](crate::client::fluent_builders::GetApiAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::GetApiAssociation::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::GetApiAssociation::set_domain_name): <p>The domain name.</p>
                            /// - On success, responds with [`GetApiAssociationOutput`](crate::output::GetApiAssociationOutput) with field(s):
    ///   - [`api_association(Option<ApiAssociation>)`](crate::output::GetApiAssociationOutput::api_association): <p>The <code>ApiAssociation</code> object.</p>
                            /// - On failure, responds with [`SdkError<GetApiAssociationError>`](crate::error::GetApiAssociationError)
    pub fn get_api_association(&self) -> crate::client::fluent_builders::GetApiAssociation {
                                crate::client::fluent_builders::GetApiAssociation::new(self.handle.clone())
                            }
}

