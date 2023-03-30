// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteInvitations`](crate::client::fluent_builders::DeleteInvitations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::DeleteInvitations::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteInvitations::set_account_ids): <p>The list of the account IDs that sent the invitations to delete.</p>
                            /// - On success, responds with [`DeleteInvitationsOutput`](crate::output::DeleteInvitationsOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<Result>>)`](crate::output::DeleteInvitationsOutput::unprocessed_accounts): <p>The list of Amazon Web Services accounts for which the invitations were not deleted. For each account, the list includes the account ID and the email address.</p>
                            /// - On failure, responds with [`SdkError<DeleteInvitationsError>`](crate::error::DeleteInvitationsError)
    pub fn delete_invitations(&self) -> crate::client::fluent_builders::DeleteInvitations {
                                crate::client::fluent_builders::DeleteInvitations::new(self.handle.clone())
                            }
}

