// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListUsers`](crate::client::fluent_builders::ListUsers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUsers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::client::fluent_builders::ListUsers::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::client::fluent_builders::ListUsers::set_identity_store_id): <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains numbers and lower case letters. This value is generated at the time that a new identity store is created.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListUsers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListUsers::set_max_results): <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> requests to specify how many results to return in one page. The length limit is 50 characters.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListUsers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListUsers::set_next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListUsers::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListUsers::set_filters): <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> requests. </p>
                            /// - On success, responds with [`ListUsersOutput`](crate::output::ListUsersOutput) with field(s):
    ///   - [`users(Option<Vec<User>>)`](crate::output::ListUsersOutput::users): <p>A list of <code>User</code> objects in the identity store.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListUsersOutput::next_token): <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
                            /// - On failure, responds with [`SdkError<ListUsersError>`](crate::error::ListUsersError)
    pub fn list_users(&self) -> crate::client::fluent_builders::ListUsers {
                                crate::client::fluent_builders::ListUsers::new(self.handle.clone())
                            }
}

