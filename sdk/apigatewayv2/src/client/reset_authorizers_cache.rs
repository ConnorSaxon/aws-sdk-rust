// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ResetAuthorizersCache`](crate::client::fluent_builders::ResetAuthorizersCache) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::ResetAuthorizersCache::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::ResetAuthorizersCache::set_api_id): <p>The API identifier.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::ResetAuthorizersCache::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::ResetAuthorizersCache::set_stage_name): <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
                            /// - On success, responds with [`ResetAuthorizersCacheOutput`](crate::output::ResetAuthorizersCacheOutput)
                            /// - On failure, responds with [`SdkError<ResetAuthorizersCacheError>`](crate::error::ResetAuthorizersCacheError)
    pub fn reset_authorizers_cache(&self) -> crate::client::fluent_builders::ResetAuthorizersCache {
                                crate::client::fluent_builders::ResetAuthorizersCache::new(self.handle.clone())
                            }
}

