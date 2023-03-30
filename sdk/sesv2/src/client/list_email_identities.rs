// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEmailIdentities`](crate::client::fluent_builders::ListEmailIdentities) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEmailIdentities::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEmailIdentities::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEmailIdentities::set_next_token): <p>A token returned from a previous call to <code>ListEmailIdentities</code> to indicate the position in the list of identities.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListEmailIdentities::page_size) / [`set_page_size(Option<i32>)`](crate::client::fluent_builders::ListEmailIdentities::set_page_size): <p>The number of results to show in a single call to <code>ListEmailIdentities</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>  <p>The value you specify has to be at least 0, and can be no more than 1000.</p>
                            /// - On success, responds with [`ListEmailIdentitiesOutput`](crate::output::ListEmailIdentitiesOutput) with field(s):
    ///   - [`email_identities(Option<Vec<IdentityInfo>>)`](crate::output::ListEmailIdentitiesOutput::email_identities): <p>An array that includes all of the email identities associated with your Amazon Web Services account.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListEmailIdentitiesOutput::next_token): <p>A token that indicates that there are additional configuration sets to list. To view additional configuration sets, issue another request to <code>ListEmailIdentities</code>, and pass this token in the <code>NextToken</code> parameter.</p>
                            /// - On failure, responds with [`SdkError<ListEmailIdentitiesError>`](crate::error::ListEmailIdentitiesError)
    pub fn list_email_identities(&self) -> crate::client::fluent_builders::ListEmailIdentities {
                                crate::client::fluent_builders::ListEmailIdentities::new(self.handle.clone())
                            }
}

