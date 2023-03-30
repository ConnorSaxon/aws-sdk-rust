// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateOriginRequestPolicy`](crate::client::fluent_builders::CreateOriginRequestPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`origin_request_policy_config(OriginRequestPolicyConfig)`](crate::client::fluent_builders::CreateOriginRequestPolicy::origin_request_policy_config) / [`set_origin_request_policy_config(Option<OriginRequestPolicyConfig>)`](crate::client::fluent_builders::CreateOriginRequestPolicy::set_origin_request_policy_config): <p>An origin request policy configuration.</p>
                            /// - On success, responds with [`CreateOriginRequestPolicyOutput`](crate::output::CreateOriginRequestPolicyOutput) with field(s):
    ///   - [`origin_request_policy(Option<OriginRequestPolicy>)`](crate::output::CreateOriginRequestPolicyOutput::origin_request_policy): <p>An origin request policy.</p>
    ///   - [`location(Option<String>)`](crate::output::CreateOriginRequestPolicyOutput::location): <p>The fully qualified URI of the origin request policy just created.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::CreateOriginRequestPolicyOutput::e_tag): <p>The current version of the origin request policy.</p>
                            /// - On failure, responds with [`SdkError<CreateOriginRequestPolicyError>`](crate::error::CreateOriginRequestPolicyError)
    pub fn create_origin_request_policy(&self) -> crate::client::fluent_builders::CreateOriginRequestPolicy {
                                crate::client::fluent_builders::CreateOriginRequestPolicy::new(self.handle.clone())
                            }
}

