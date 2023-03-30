// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateMemberAccount`](crate::client::fluent_builders::DisassociateMemberAccount) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateMemberAccount::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::DisassociateMemberAccount::set_member_account_id): <p>(Discontinued) The ID of the member account that you want to remove from Amazon Macie Classic.</p>
                            /// - On success, responds with [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
                            /// - On failure, responds with [`SdkError<DisassociateMemberAccountError>`](crate::error::DisassociateMemberAccountError)
    pub fn disassociate_member_account(&self) -> crate::client::fluent_builders::DisassociateMemberAccount {
                                crate::client::fluent_builders::DisassociateMemberAccount::new(self.handle.clone())
                            }
}

