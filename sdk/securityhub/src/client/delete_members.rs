// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMembers`](crate::client::fluent_builders::DeleteMembers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::DeleteMembers::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteMembers::set_account_ids): <p>The list of account IDs for the member accounts to delete.</p>
                            /// - On success, responds with [`DeleteMembersOutput`](crate::output::DeleteMembersOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<Result>>)`](crate::output::DeleteMembersOutput::unprocessed_accounts): <p>The list of Amazon Web Services accounts that were not deleted. For each account, the list includes the account ID and the email address.</p>
                            /// - On failure, responds with [`SdkError<DeleteMembersError>`](crate::error::DeleteMembersError)
    pub fn delete_members(&self) -> crate::client::fluent_builders::DeleteMembers {
                                crate::client::fluent_builders::DeleteMembers::new(self.handle.clone())
                            }
}

