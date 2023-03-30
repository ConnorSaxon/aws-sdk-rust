// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePolicy`](crate::client::fluent_builders::DeletePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::client::fluent_builders::DeletePolicy::policy_arn) / [`set_policy_arn(Option<String>)`](crate::client::fluent_builders::DeletePolicy::set_policy_arn): <p>The Amazon Resource Name (ARN) of the IAM policy you want to delete.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
                            /// - On success, responds with [`DeletePolicyOutput`](crate::output::DeletePolicyOutput)
                            /// - On failure, responds with [`SdkError<DeletePolicyError>`](crate::error::DeletePolicyError)
    pub fn delete_policy(&self) -> crate::client::fluent_builders::DeletePolicy {
                                crate::client::fluent_builders::DeletePolicy::new(self.handle.clone())
                            }
}

