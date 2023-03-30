// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAggregationAuthorizations`](crate::client::fluent_builders::DescribeAggregationAuthorizations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeAggregationAuthorizations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeAggregationAuthorizations::limit) / [`set_limit(i32)`](crate::client::fluent_builders::DescribeAggregationAuthorizations::set_limit): <p>The maximum number of AggregationAuthorizations returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeAggregationAuthorizations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeAggregationAuthorizations::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
                            /// - On success, responds with [`DescribeAggregationAuthorizationsOutput`](crate::output::DescribeAggregationAuthorizationsOutput) with field(s):
    ///   - [`aggregation_authorizations(Option<Vec<AggregationAuthorization>>)`](crate::output::DescribeAggregationAuthorizationsOutput::aggregation_authorizations): <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeAggregationAuthorizationsOutput::next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
                            /// - On failure, responds with [`SdkError<DescribeAggregationAuthorizationsError>`](crate::error::DescribeAggregationAuthorizationsError)
    pub fn describe_aggregation_authorizations(&self) -> crate::client::fluent_builders::DescribeAggregationAuthorizations {
                                crate::client::fluent_builders::DescribeAggregationAuthorizations::new(self.handle.clone())
                            }
}

