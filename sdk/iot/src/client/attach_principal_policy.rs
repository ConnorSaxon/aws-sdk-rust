// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AttachPrincipalPolicy`](crate::client::fluent_builders::AttachPrincipalPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_name(impl Into<String>)`](crate::client::fluent_builders::AttachPrincipalPolicy::policy_name) / [`set_policy_name(Option<String>)`](crate::client::fluent_builders::AttachPrincipalPolicy::set_policy_name): <p>The policy name.</p>
    ///   - [`principal(impl Into<String>)`](crate::client::fluent_builders::AttachPrincipalPolicy::principal) / [`set_principal(Option<String>)`](crate::client::fluent_builders::AttachPrincipalPolicy::set_principal): <p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>
                            /// - On success, responds with [`AttachPrincipalPolicyOutput`](crate::output::AttachPrincipalPolicyOutput)
                            /// - On failure, responds with [`SdkError<AttachPrincipalPolicyError>`](crate::error::AttachPrincipalPolicyError)
    #[deprecated]
    pub fn attach_principal_policy(&self) -> crate::client::fluent_builders::AttachPrincipalPolicy {
                                crate::client::fluent_builders::AttachPrincipalPolicy::new(self.handle.clone())
                            }
}

