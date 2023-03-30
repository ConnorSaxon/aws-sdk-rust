// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterIdentityProvider`](crate::client::fluent_builders::DeregisterIdentityProvider) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_provider(IdentityProvider)`](crate::client::fluent_builders::DeregisterIdentityProvider::identity_provider) / [`set_identity_provider(Option<IdentityProvider>)`](crate::client::fluent_builders::DeregisterIdentityProvider::set_identity_provider): <p>An object that specifies details for the identity provider.</p>
    ///   - [`product(impl Into<String>)`](crate::client::fluent_builders::DeregisterIdentityProvider::product) / [`set_product(Option<String>)`](crate::client::fluent_builders::DeregisterIdentityProvider::set_product): <p>The name of the user-based subscription product.</p>
                            /// - On success, responds with [`DeregisterIdentityProviderOutput`](crate::output::DeregisterIdentityProviderOutput) with field(s):
    ///   - [`identity_provider_summary(Option<IdentityProviderSummary>)`](crate::output::DeregisterIdentityProviderOutput::identity_provider_summary): <p>Metadata that describes the results of an identity provider operation.</p>
                            /// - On failure, responds with [`SdkError<DeregisterIdentityProviderError>`](crate::error::DeregisterIdentityProviderError)
    pub fn deregister_identity_provider(&self) -> crate::client::fluent_builders::DeregisterIdentityProvider {
                                crate::client::fluent_builders::DeregisterIdentityProvider::new(self.handle.clone())
                            }
}

