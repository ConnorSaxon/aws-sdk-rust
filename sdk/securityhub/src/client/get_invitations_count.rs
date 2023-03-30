// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetInvitationsCount`](crate::client::fluent_builders::GetInvitationsCount) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetInvitationsCount::send) it.
                            /// - On success, responds with [`GetInvitationsCountOutput`](crate::output::GetInvitationsCountOutput) with field(s):
    ///   - [`invitations_count(i32)`](crate::output::GetInvitationsCountOutput::invitations_count): <p>The number of all membership invitations sent to this Security Hub member account, not including the currently accepted invitation.</p>
                            /// - On failure, responds with [`SdkError<GetInvitationsCountError>`](crate::error::GetInvitationsCountError)
    pub fn get_invitations_count(&self) -> crate::client::fluent_builders::GetInvitationsCount {
                                crate::client::fluent_builders::GetInvitationsCount::new(self.handle.clone())
                            }
}

