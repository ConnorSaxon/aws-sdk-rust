// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAccessPolicy`](crate::client::fluent_builders::UpdateAccessPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`r#type(AccessPolicyType)`](crate::client::fluent_builders::UpdateAccessPolicy::type) / [`set_type(Option<AccessPolicyType>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_type): <p>The type of policy.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_name): <p>The name of the policy.</p>
    ///   - [`policy_version(impl Into<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::policy_version) / [`set_policy_version(Option<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_policy_version): <p>The version of the policy being updated.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_description): <p>A description of the policy. Typically used to store information about the permissions defined in the policy.</p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_policy): <p>The JSON policy document to use as the content for the policy.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateAccessPolicy::set_client_token): <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
                            /// - On success, responds with [`UpdateAccessPolicyOutput`](crate::output::UpdateAccessPolicyOutput) with field(s):
    ///   - [`access_policy_detail(Option<AccessPolicyDetail>)`](crate::output::UpdateAccessPolicyOutput::access_policy_detail): <p>Details about the updated access policy.</p>
                            /// - On failure, responds with [`SdkError<UpdateAccessPolicyError>`](crate::error::UpdateAccessPolicyError)
    pub fn update_access_policy(&self) -> crate::client::fluent_builders::UpdateAccessPolicy {
                                crate::client::fluent_builders::UpdateAccessPolicy::new(self.handle.clone())
                            }
}

