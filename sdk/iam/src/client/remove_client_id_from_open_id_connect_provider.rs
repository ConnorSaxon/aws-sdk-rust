// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveClientIDFromOpenIDConnectProvider`](crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`open_id_connect_provider_arn(impl Into<String>)`](crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider::open_id_connect_provider_arn) / [`set_open_id_connect_provider_arn(Option<String>)`](crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider::set_open_id_connect_provider_arn): <p>The Amazon Resource Name (ARN) of the IAM OIDC provider resource to remove the client ID from. You can get a list of OIDC provider ARNs by using the <code>ListOpenIDConnectProviders</code> operation.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`client_id(impl Into<String>)`](crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider::client_id) / [`set_client_id(Option<String>)`](crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider::set_client_id): <p>The client ID (also known as audience) to remove from the IAM OIDC provider resource. For more information about client IDs, see <code>CreateOpenIDConnectProvider</code>.</p>
                            /// - On success, responds with [`RemoveClientIdFromOpenIdConnectProviderOutput`](crate::output::RemoveClientIdFromOpenIdConnectProviderOutput)
                            /// - On failure, responds with [`SdkError<RemoveClientIDFromOpenIDConnectProviderError>`](crate::error::RemoveClientIDFromOpenIDConnectProviderError)
    pub fn remove_client_id_from_open_id_connect_provider(&self) -> crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider {
                                crate::client::fluent_builders::RemoveClientIDFromOpenIDConnectProvider::new(self.handle.clone())
                            }
}

