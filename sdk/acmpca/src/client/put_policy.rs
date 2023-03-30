// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutPolicy`](crate::client::fluent_builders::PutPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::PutPolicy::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::PutPolicy::set_resource_arn): <p>The Amazon Resource Number (ARN) of the private CA to associate with the policy. The ARN of the CA can be found by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action.</p>  <p></p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::PutPolicy::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::PutPolicy::set_policy): <p>The path and file name of a JSON-formatted IAM policy to attach to the specified private CA resource. If this policy does not contain all required statements or if it includes any statement that is not allowed, the <code>PutPolicy</code> action returns an <code>InvalidPolicyException</code>. For information about IAM policy and statement structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json">Overview of JSON Policies</a>.</p>
                            /// - On success, responds with [`PutPolicyOutput`](crate::output::PutPolicyOutput)
                            /// - On failure, responds with [`SdkError<PutPolicyError>`](crate::error::PutPolicyError)
    pub fn put_policy(&self) -> crate::client::fluent_builders::PutPolicy {
                                crate::client::fluent_builders::PutPolicy::new(self.handle.clone())
                            }
}

