// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateResourcePolicy`](crate::client::fluent_builders::CreateResourcePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::CreateResourcePolicy::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::CreateResourcePolicy::set_resource_arn): <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::CreateResourcePolicy::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::CreateResourcePolicy::set_policy): <p>A resource policy to add to the resource. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy reference </a>. </p>  <p>If the policy isn't valid, Amazon Lex returns a validation exception.</p>
                            /// - On success, responds with [`CreateResourcePolicyOutput`](crate::output::CreateResourcePolicyOutput) with field(s):
    ///   - [`resource_arn(Option<String>)`](crate::output::CreateResourcePolicyOutput::resource_arn): <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy was attached to.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::CreateResourcePolicyOutput::revision_id): <p>The current revision of the resource policy. Use the revision ID to make sure that you are updating the most current version of a resource policy when you add a policy statement to a resource, delete a resource, or update a resource.</p>
                            /// - On failure, responds with [`SdkError<CreateResourcePolicyError>`](crate::error::CreateResourcePolicyError)
    pub fn create_resource_policy(&self) -> crate::client::fluent_builders::CreateResourcePolicy {
                                crate::client::fluent_builders::CreateResourcePolicy::new(self.handle.clone())
                            }
}

