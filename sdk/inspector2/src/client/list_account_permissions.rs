// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAccountPermissions`](crate::client::fluent_builders::ListAccountPermissions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAccountPermissions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`service(Service)`](crate::client::fluent_builders::ListAccountPermissions::service) / [`set_service(Option<Service>)`](crate::client::fluent_builders::ListAccountPermissions::set_service): <p>The service scan type to check permissions for.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAccountPermissions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAccountPermissions::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAccountPermissions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAccountPermissions::set_next_token): <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
                            /// - On success, responds with [`ListAccountPermissionsOutput`](crate::output::ListAccountPermissionsOutput) with field(s):
    ///   - [`permissions(Option<Vec<Permission>>)`](crate::output::ListAccountPermissionsOutput::permissions): <p>Contains details on the permissions an account has to configure Amazon Inspector.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAccountPermissionsOutput::next_token): <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
                            /// - On failure, responds with [`SdkError<ListAccountPermissionsError>`](crate::error::ListAccountPermissionsError)
    pub fn list_account_permissions(&self) -> crate::client::fluent_builders::ListAccountPermissions {
                                crate::client::fluent_builders::ListAccountPermissions::new(self.handle.clone())
                            }
}

