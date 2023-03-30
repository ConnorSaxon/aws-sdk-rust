// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVerifiedAccessEndpointPolicy`](crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`verified_access_endpoint_id(impl Into<String>)`](crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy::verified_access_endpoint_id) / [`set_verified_access_endpoint_id(Option<String>)`](crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy::set_verified_access_endpoint_id): <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`GetVerifiedAccessEndpointPolicyOutput`](crate::output::GetVerifiedAccessEndpointPolicyOutput) with field(s):
    ///   - [`policy_enabled(Option<bool>)`](crate::output::GetVerifiedAccessEndpointPolicyOutput::policy_enabled): <p>The status of the Verified Access policy.</p>
    ///   - [`policy_document(Option<String>)`](crate::output::GetVerifiedAccessEndpointPolicyOutput::policy_document): <p>The Amazon Web Services Verified Access policy document.</p>
                            /// - On failure, responds with [`SdkError<GetVerifiedAccessEndpointPolicyError>`](crate::error::GetVerifiedAccessEndpointPolicyError)
    pub fn get_verified_access_endpoint_policy(&self) -> crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy {
                                crate::client::fluent_builders::GetVerifiedAccessEndpointPolicy::new(self.handle.clone())
                            }
}

