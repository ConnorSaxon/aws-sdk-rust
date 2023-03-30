// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteIdentityProvider`](crate::client::fluent_builders::DeleteIdentityProvider) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::DeleteIdentityProvider::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::DeleteIdentityProvider::set_user_pool_id): <p>The user pool ID.</p>
    ///   - [`provider_name(impl Into<String>)`](crate::client::fluent_builders::DeleteIdentityProvider::provider_name) / [`set_provider_name(Option<String>)`](crate::client::fluent_builders::DeleteIdentityProvider::set_provider_name): <p>The IdP name.</p>
                            /// - On success, responds with [`DeleteIdentityProviderOutput`](crate::output::DeleteIdentityProviderOutput)
                            /// - On failure, responds with [`SdkError<DeleteIdentityProviderError>`](crate::error::DeleteIdentityProviderError)
    pub fn delete_identity_provider(&self) -> crate::client::fluent_builders::DeleteIdentityProvider {
                                crate::client::fluent_builders::DeleteIdentityProvider::new(self.handle.clone())
                            }
}

