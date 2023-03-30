// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAccountAttributes`](crate::client::fluent_builders::DescribeAccountAttributes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeAccountAttributes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeAccountAttributes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeAccountAttributes::set_next_token): <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeAccountAttributes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeAccountAttributes::set_max_results): <p>The maximum number of results to return per each request.</p>
                            /// - On success, responds with [`DescribeAccountAttributesOutput`](crate::output::DescribeAccountAttributesOutput) with field(s):
    ///   - [`account_attributes(Option<Vec<AccountAttribute>>)`](crate::output::DescribeAccountAttributesOutput::account_attributes): <p>An array of AccountAttributes objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeAccountAttributesOutput::next_token): <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p>
                            /// - On failure, responds with [`SdkError<DescribeAccountAttributesError>`](crate::error::DescribeAccountAttributesError)
    pub fn describe_account_attributes(&self) -> crate::client::fluent_builders::DescribeAccountAttributes {
                                crate::client::fluent_builders::DescribeAccountAttributes::new(self.handle.clone())
                            }
}

