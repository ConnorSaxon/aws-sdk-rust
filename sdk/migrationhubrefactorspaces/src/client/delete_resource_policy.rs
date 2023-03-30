// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteResourcePolicy`](crate::client::fluent_builders::DeleteResourcePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::identifier) / [`set_identifier(Option<String>)`](crate::client::fluent_builders::DeleteResourcePolicy::set_identifier): <p>Amazon Resource Name (ARN) of the resource associated with the policy. </p>
                            /// - On success, responds with [`DeleteResourcePolicyOutput`](crate::output::DeleteResourcePolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteResourcePolicyError>`](crate::error::DeleteResourcePolicyError)
    pub fn delete_resource_policy(&self) -> crate::client::fluent_builders::DeleteResourcePolicy {
                                crate::client::fluent_builders::DeleteResourcePolicy::new(self.handle.clone())
                            }
}

