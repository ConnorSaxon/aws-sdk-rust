// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListIdentityProviders`](crate::client::fluent_builders::ListIdentityProviders) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListIdentityProviders::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::ListIdentityProviders::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::ListIdentityProviders::set_user_pool_id): <p>The user pool ID.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListIdentityProviders::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListIdentityProviders::set_max_results): <p>The maximum number of IdPs to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListIdentityProviders::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListIdentityProviders::set_next_token): <p>A pagination token.</p>
                            /// - On success, responds with [`ListIdentityProvidersOutput`](crate::output::ListIdentityProvidersOutput) with field(s):
    ///   - [`providers(Option<Vec<ProviderDescription>>)`](crate::output::ListIdentityProvidersOutput::providers): <p>A list of IdP objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListIdentityProvidersOutput::next_token): <p>A pagination token.</p>
                            /// - On failure, responds with [`SdkError<ListIdentityProvidersError>`](crate::error::ListIdentityProvidersError)
    pub fn list_identity_providers(&self) -> crate::client::fluent_builders::ListIdentityProviders {
                                crate::client::fluent_builders::ListIdentityProviders::new(self.handle.clone())
                            }
}

