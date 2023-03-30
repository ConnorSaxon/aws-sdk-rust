// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMembership`](crate::client::fluent_builders::GetMembership) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::client::fluent_builders::GetMembership::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::client::fluent_builders::GetMembership::set_membership_identifier): <p>The identifier for a membership resource.</p>
                            /// - On success, responds with [`GetMembershipOutput`](crate::output::GetMembershipOutput) with field(s):
    ///   - [`membership(Option<Membership>)`](crate::output::GetMembershipOutput::membership): <p>The membership retrieved for the provided identifier.</p>
                            /// - On failure, responds with [`SdkError<GetMembershipError>`](crate::error::GetMembershipError)
    pub fn get_membership(&self) -> crate::client::fluent_builders::GetMembership {
                                crate::client::fluent_builders::GetMembership::new(self.handle.clone())
                            }
}

