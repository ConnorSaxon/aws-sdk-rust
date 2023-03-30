// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListUsers`](crate::client::fluent_builders::ListUsers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUsers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::ListUsers::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::ListUsers::set_organization_id): <p>The identifier for the organization under which the users exist.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListUsers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListUsers::set_next_token): <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListUsers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListUsers::set_max_results): <p>The maximum number of results to return in a single call.</p>
                            /// - On success, responds with [`ListUsersOutput`](crate::output::ListUsersOutput) with field(s):
    ///   - [`users(Option<Vec<User>>)`](crate::output::ListUsersOutput::users): <p>The overview of users for an organization.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListUsersOutput::next_token): <p> The token to use to retrieve the next page of results. This value is `null` when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListUsersError>`](crate::error::ListUsersError)
    pub fn list_users(&self) -> crate::client::fluent_builders::ListUsers {
                                crate::client::fluent_builders::ListUsers::new(self.handle.clone())
                            }
}

