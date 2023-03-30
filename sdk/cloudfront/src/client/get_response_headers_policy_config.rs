// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResponseHeadersPolicyConfig`](crate::client::fluent_builders::GetResponseHeadersPolicyConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetResponseHeadersPolicyConfig::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetResponseHeadersPolicyConfig::set_id): <p>The identifier for the response headers policy.</p>  <p>If the response headers policy is attached to a distribution's cache behavior, you can get the policy's identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the response headers policy is not attached to a cache behavior, you can get the identifier using <code>ListResponseHeadersPolicies</code>.</p>
                            /// - On success, responds with [`GetResponseHeadersPolicyConfigOutput`](crate::output::GetResponseHeadersPolicyConfigOutput) with field(s):
    ///   - [`response_headers_policy_config(Option<ResponseHeadersPolicyConfig>)`](crate::output::GetResponseHeadersPolicyConfigOutput::response_headers_policy_config): <p>Contains a response headers policy.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::GetResponseHeadersPolicyConfigOutput::e_tag): <p>The version identifier for the current version of the response headers policy.</p>
                            /// - On failure, responds with [`SdkError<GetResponseHeadersPolicyConfigError>`](crate::error::GetResponseHeadersPolicyConfigError)
    pub fn get_response_headers_policy_config(&self) -> crate::client::fluent_builders::GetResponseHeadersPolicyConfig {
                                crate::client::fluent_builders::GetResponseHeadersPolicyConfig::new(self.handle.clone())
                            }
}

