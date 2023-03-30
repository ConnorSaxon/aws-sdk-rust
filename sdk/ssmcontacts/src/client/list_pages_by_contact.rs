// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPagesByContact`](crate::client::fluent_builders::ListPagesByContact) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPagesByContact::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`contact_id(impl Into<String>)`](crate::client::fluent_builders::ListPagesByContact::contact_id) / [`set_contact_id(Option<String>)`](crate::client::fluent_builders::ListPagesByContact::set_contact_id): <p>The Amazon Resource Name (ARN) of the contact you are retrieving engagements for.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPagesByContact::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPagesByContact::set_next_token): <p>The pagination token to continue to the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPagesByContact::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPagesByContact::set_max_results): <p>The maximum number of engagements to contact channels to list per page of results. </p>
                            /// - On success, responds with [`ListPagesByContactOutput`](crate::output::ListPagesByContactOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListPagesByContactOutput::next_token): <p>The pagination token to continue to the next page of results.</p>
    ///   - [`pages(Option<Vec<Page>>)`](crate::output::ListPagesByContactOutput::pages): <p>The list of engagements to a contact's contact channel.</p>
                            /// - On failure, responds with [`SdkError<ListPagesByContactError>`](crate::error::ListPagesByContactError)
    pub fn list_pages_by_contact(&self) -> crate::client::fluent_builders::ListPagesByContact {
                                crate::client::fluent_builders::ListPagesByContact::new(self.handle.clone())
                            }
}

