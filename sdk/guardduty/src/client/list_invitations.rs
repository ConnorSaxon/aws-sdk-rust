// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListInvitations`](crate::client::fluent_builders::ListInvitations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListInvitations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListInvitations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListInvitations::set_max_results): <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 50. The maximum value is 50.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListInvitations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListInvitations::set_next_token): <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action, fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
                            /// - On success, responds with [`ListInvitationsOutput`](crate::output::ListInvitationsOutput) with field(s):
    ///   - [`invitations(Option<Vec<Invitation>>)`](crate::output::ListInvitationsOutput::invitations): <p>A list of invitation descriptions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListInvitationsOutput::next_token): <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
                            /// - On failure, responds with [`SdkError<ListInvitationsError>`](crate::error::ListInvitationsError)
    pub fn list_invitations(&self) -> crate::client::fluent_builders::ListInvitations {
                                crate::client::fluent_builders::ListInvitations::new(self.handle.clone())
                            }
}

