// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListUserPoolClients`](crate::client::fluent_builders::ListUserPoolClients) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUserPoolClients::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::ListUserPoolClients::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::ListUserPoolClients::set_user_pool_id): <p>The user pool ID for the user pool where you want to list user pool clients.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListUserPoolClients::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListUserPoolClients::set_max_results): <p>The maximum number of results you want the request to return when listing the user pool clients.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListUserPoolClients::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListUserPoolClients::set_next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
                            /// - On success, responds with [`ListUserPoolClientsOutput`](crate::output::ListUserPoolClientsOutput) with field(s):
    ///   - [`user_pool_clients(Option<Vec<UserPoolClientDescription>>)`](crate::output::ListUserPoolClientsOutput::user_pool_clients): <p>The user pool clients in the response that lists user pool clients.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListUserPoolClientsOutput::next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
                            /// - On failure, responds with [`SdkError<ListUserPoolClientsError>`](crate::error::ListUserPoolClientsError)
    pub fn list_user_pool_clients(&self) -> crate::client::fluent_builders::ListUserPoolClients {
                                crate::client::fluent_builders::ListUserPoolClients::new(self.handle.clone())
                            }
}

