// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeInstanceCreditSpecifications`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>  </ul>
    ///   - [`instance_ids(Vec<String>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::set_instance_ids): <p>The instance IDs.</p>  <p>Default: Describes all your instances.</p>  <p>Constraints: Maximum 1000 explicitly specified instance IDs.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::set_max_results): <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. This value can be between 5 and 1000. You cannot specify this parameter and the instance IDs parameter in the same call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeInstanceCreditSpecifications::set_next_token): <p>The token to retrieve the next page of results.</p>
                            /// - On success, responds with [`DescribeInstanceCreditSpecificationsOutput`](crate::output::DescribeInstanceCreditSpecificationsOutput) with field(s):
    ///   - [`instance_credit_specifications(Option<Vec<InstanceCreditSpecification>>)`](crate::output::DescribeInstanceCreditSpecificationsOutput::instance_credit_specifications): <p>Information about the credit option for CPU usage of an instance.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeInstanceCreditSpecificationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeInstanceCreditSpecificationsError>`](crate::error::DescribeInstanceCreditSpecificationsError)
    pub fn describe_instance_credit_specifications(&self) -> crate::client::fluent_builders::DescribeInstanceCreditSpecifications {
                                crate::client::fluent_builders::DescribeInstanceCreditSpecifications::new(self.handle.clone())
                            }
}

