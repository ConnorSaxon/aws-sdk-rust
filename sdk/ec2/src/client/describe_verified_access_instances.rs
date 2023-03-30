// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeVerifiedAccessInstances`](crate::client::fluent_builders::DescribeVerifiedAccessInstances) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`verified_access_instance_ids(Vec<String>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::verified_access_instance_ids) / [`set_verified_access_instance_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::set_verified_access_instance_ids): <p>The IDs of the Amazon Web Services Verified Access instances.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::set_filters): <p>One or more filters. Filter names and values are case-sensitive.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeVerifiedAccessInstances::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeVerifiedAccessInstancesOutput`](crate::output::DescribeVerifiedAccessInstancesOutput) with field(s):
    ///   - [`verified_access_instances(Option<Vec<VerifiedAccessInstance>>)`](crate::output::DescribeVerifiedAccessInstancesOutput::verified_access_instances): <p>The IDs of the Amazon Web Services Verified Access instances.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeVerifiedAccessInstancesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeVerifiedAccessInstancesError>`](crate::error::DescribeVerifiedAccessInstancesError)
    pub fn describe_verified_access_instances(&self) -> crate::client::fluent_builders::DescribeVerifiedAccessInstances {
                                crate::client::fluent_builders::DescribeVerifiedAccessInstances::new(self.handle.clone())
                            }
}

