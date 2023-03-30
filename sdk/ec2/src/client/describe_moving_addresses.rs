// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeMovingAddresses`](crate::client::fluent_builders::DescribeMovingAddresses) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeMovingAddresses::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeMovingAddresses::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeMovingAddresses::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>moving-status</code> - The status of the Elastic IP address (<code>MovingToVpc</code> | <code>RestoringToClassic</code>).</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeMovingAddresses::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeMovingAddresses::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeMovingAddresses::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeMovingAddresses::set_max_results): <p>The maximum number of results to return for the request in a single page. The remaining results of the initial request can be seen by sending another request with the returned <code>NextToken</code> value. This value can be between 5 and 1000; if <code>MaxResults</code> is given a value outside of this range, an error is returned.</p>  <p>Default: If no value is provided, the default is 1000.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeMovingAddresses::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeMovingAddresses::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`public_ips(Vec<String>)`](crate::client::fluent_builders::DescribeMovingAddresses::public_ips) / [`set_public_ips(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeMovingAddresses::set_public_ips): <p>One or more Elastic IP addresses.</p>
                            /// - On success, responds with [`DescribeMovingAddressesOutput`](crate::output::DescribeMovingAddressesOutput) with field(s):
    ///   - [`moving_address_statuses(Option<Vec<MovingAddressStatus>>)`](crate::output::DescribeMovingAddressesOutput::moving_address_statuses): <p>The status for each Elastic IP address.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeMovingAddressesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeMovingAddressesError>`](crate::error::DescribeMovingAddressesError)
    pub fn describe_moving_addresses(&self) -> crate::client::fluent_builders::DescribeMovingAddresses {
                                crate::client::fluent_builders::DescribeMovingAddresses::new(self.handle.clone())
                            }
}

