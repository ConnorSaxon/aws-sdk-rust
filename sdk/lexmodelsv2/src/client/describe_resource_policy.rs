// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeResourcePolicy`](crate::client::fluent_builders::DescribeResourcePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeResourcePolicy::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::DescribeResourcePolicy::set_resource_arn): <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
                            /// - On success, responds with [`DescribeResourcePolicyOutput`](crate::output::DescribeResourcePolicyOutput) with field(s):
    ///   - [`resource_arn(Option<String>)`](crate::output::DescribeResourcePolicyOutput::resource_arn): <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
    ///   - [`policy(Option<String>)`](crate::output::DescribeResourcePolicyOutput::policy): <p>The JSON structure that contains the resource policy. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy reference </a>.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::DescribeResourcePolicyOutput::revision_id): <p>The current revision of the resource policy. Use the revision ID to make sure that you are updating the most current version of a resource policy when you add a policy statement to a resource, delete a resource, or update a resource.</p>
                            /// - On failure, responds with [`SdkError<DescribeResourcePolicyError>`](crate::error::DescribeResourcePolicyError)
    pub fn describe_resource_policy(&self) -> crate::client::fluent_builders::DescribeResourcePolicy {
                                crate::client::fluent_builders::DescribeResourcePolicy::new(self.handle.clone())
                            }
}

