// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRolePolicy`](crate::client::fluent_builders::DeleteRolePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRolePolicy::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::DeleteRolePolicy::set_role_name): <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`policy_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRolePolicy::policy_name) / [`set_policy_name(Option<String>)`](crate::client::fluent_builders::DeleteRolePolicy::set_policy_name): <p>The name of the inline policy to delete from the specified IAM role.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
                            /// - On success, responds with [`DeleteRolePolicyOutput`](crate::output::DeleteRolePolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteRolePolicyError>`](crate::error::DeleteRolePolicyError)
    pub fn delete_role_policy(&self) -> crate::client::fluent_builders::DeleteRolePolicy {
                                crate::client::fluent_builders::DeleteRolePolicy::new(self.handle.clone())
                            }
}

