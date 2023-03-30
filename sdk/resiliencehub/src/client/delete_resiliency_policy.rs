// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteResiliencyPolicy`](crate::client::fluent_builders::DeleteResiliencyPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteResiliencyPolicy::policy_arn) / [`set_policy_arn(Option<String>)`](crate::client::fluent_builders::DeleteResiliencyPolicy::set_policy_arn): <p>The Amazon Resource Name (ARN) of the resiliency policy. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:resiliency-policy/<code>policy-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::DeleteResiliencyPolicy::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::DeleteResiliencyPolicy::set_client_token): <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. You should not reuse the same client token for other API requests.</p>
                            /// - On success, responds with [`DeleteResiliencyPolicyOutput`](crate::output::DeleteResiliencyPolicyOutput) with field(s):
    ///   - [`policy_arn(Option<String>)`](crate::output::DeleteResiliencyPolicyOutput::policy_arn): <p>The Amazon Resource Name (ARN) of the resiliency policy. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:resiliency-policy/<code>policy-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
                            /// - On failure, responds with [`SdkError<DeleteResiliencyPolicyError>`](crate::error::DeleteResiliencyPolicyError)
    pub fn delete_resiliency_policy(&self) -> crate::client::fluent_builders::DeleteResiliencyPolicy {
                                crate::client::fluent_builders::DeleteResiliencyPolicy::new(self.handle.clone())
                            }
}

