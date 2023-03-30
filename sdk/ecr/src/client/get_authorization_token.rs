// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAuthorizationToken`](crate::client::fluent_builders::GetAuthorizationToken) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`registry_ids(Vec<String>)`](crate::client::fluent_builders::GetAuthorizationToken::registry_ids) / [`set_registry_ids(Option<Vec<String>>)`](crate::client::fluent_builders::GetAuthorizationToken::set_registry_ids): <p>A list of Amazon Web Services account IDs that are associated with the registries for which to get AuthorizationData objects. If you do not specify a registry, the default registry is assumed.</p>
                            /// - On success, responds with [`GetAuthorizationTokenOutput`](crate::output::GetAuthorizationTokenOutput) with field(s):
    ///   - [`authorization_data(Option<Vec<AuthorizationData>>)`](crate::output::GetAuthorizationTokenOutput::authorization_data): <p>A list of authorization token data objects that correspond to the <code>registryIds</code> values in the request.</p>
                            /// - On failure, responds with [`SdkError<GetAuthorizationTokenError>`](crate::error::GetAuthorizationTokenError)
    pub fn get_authorization_token(&self) -> crate::client::fluent_builders::GetAuthorizationToken {
                                crate::client::fluent_builders::GetAuthorizationToken::new(self.handle.clone())
                            }
}

