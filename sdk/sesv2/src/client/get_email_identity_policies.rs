// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEmailIdentityPolicies`](crate::client::fluent_builders::GetEmailIdentityPolicies) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`email_identity(impl Into<String>)`](crate::client::fluent_builders::GetEmailIdentityPolicies::email_identity) / [`set_email_identity(Option<String>)`](crate::client::fluent_builders::GetEmailIdentityPolicies::set_email_identity): <p>The email identity.</p>
                            /// - On success, responds with [`GetEmailIdentityPoliciesOutput`](crate::output::GetEmailIdentityPoliciesOutput) with field(s):
    ///   - [`policies(Option<HashMap<String, String>>)`](crate::output::GetEmailIdentityPoliciesOutput::policies): <p>A map of policy names to policies.</p>
                            /// - On failure, responds with [`SdkError<GetEmailIdentityPoliciesError>`](crate::error::GetEmailIdentityPoliciesError)
    pub fn get_email_identity_policies(&self) -> crate::client::fluent_builders::GetEmailIdentityPolicies {
                                crate::client::fluent_builders::GetEmailIdentityPolicies::new(self.handle.clone())
                            }
}

