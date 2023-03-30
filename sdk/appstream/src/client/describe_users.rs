// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeUsers`](crate::client::fluent_builders::DescribeUsers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_type(AuthenticationType)`](crate::client::fluent_builders::DescribeUsers::authentication_type) / [`set_authentication_type(Option<AuthenticationType>)`](crate::client::fluent_builders::DescribeUsers::set_authentication_type): <p>The authentication type for the users in the user pool to describe. You must specify USERPOOL.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeUsers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeUsers::set_max_results): <p>The maximum size of each page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeUsers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeUsers::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
                            /// - On success, responds with [`DescribeUsersOutput`](crate::output::DescribeUsersOutput) with field(s):
    ///   - [`users(Option<Vec<User>>)`](crate::output::DescribeUsersOutput::users): <p>Information about users in the user pool.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeUsersOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
                            /// - On failure, responds with [`SdkError<DescribeUsersError>`](crate::error::DescribeUsersError)
    pub fn describe_users(&self) -> crate::client::fluent_builders::DescribeUsers {
                                crate::client::fluent_builders::DescribeUsers::new(self.handle.clone())
                            }
}

