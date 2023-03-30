// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteResourcePolicy`](crate::client::fluent_builders::DeleteResourcePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_hash_condition(impl Into<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::policy_hash_condition) / [`set_policy_hash_condition(Option<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::set_policy_hash_condition): <p>The hash value returned when this policy was set.</p>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::set_resource_arn): <p>The ARN of the Glue resource for the resource policy to be deleted.</p>
                            /// - On success, responds with [`DeleteResourcePolicyOutput`](crate::output::DeleteResourcePolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteResourcePolicyError>`](crate::error::DeleteResourcePolicyError)
    pub fn delete_resource_policy(&self) -> crate::client::fluent_builders::DeleteResourcePolicy {
                                crate::client::fluent_builders::DeleteResourcePolicy::new(self.handle.clone())
                            }
}

