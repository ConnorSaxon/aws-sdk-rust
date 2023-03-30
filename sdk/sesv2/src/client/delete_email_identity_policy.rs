// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteEmailIdentityPolicy`](crate::client::fluent_builders::DeleteEmailIdentityPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`email_identity(impl Into<String>)`](crate::client::fluent_builders::DeleteEmailIdentityPolicy::email_identity) / [`set_email_identity(Option<String>)`](crate::client::fluent_builders::DeleteEmailIdentityPolicy::set_email_identity): <p>The email identity.</p>
    ///   - [`policy_name(impl Into<String>)`](crate::client::fluent_builders::DeleteEmailIdentityPolicy::policy_name) / [`set_policy_name(Option<String>)`](crate::client::fluent_builders::DeleteEmailIdentityPolicy::set_policy_name): <p>The name of the policy.</p>  <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p>
                            /// - On success, responds with [`DeleteEmailIdentityPolicyOutput`](crate::output::DeleteEmailIdentityPolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteEmailIdentityPolicyError>`](crate::error::DeleteEmailIdentityPolicyError)
    pub fn delete_email_identity_policy(&self) -> crate::client::fluent_builders::DeleteEmailIdentityPolicy {
                                crate::client::fluent_builders::DeleteEmailIdentityPolicy::new(self.handle.clone())
                            }
}

