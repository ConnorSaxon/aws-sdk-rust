// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeClassicLinkInstances`](crate::client::fluent_builders::DescribeClassicLinkInstances) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeClassicLinkInstances::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>group-id</code> - The ID of a VPC security group that's associated with the instance.</p> </li>   <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>vpc-id</code> - The ID of the VPC to which the instance is linked.</p> <p> <code>vpc-id</code> - The ID of the VPC that the instance is linked to.</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeClassicLinkInstances::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_ids(Vec<String>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::set_instance_ids): <p>One or more instance IDs. Must be instances linked to a VPC through ClassicLink.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeClassicLinkInstances::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>  <p>Constraint: If the value is greater than 1000, we return only 1000 items.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeClassicLinkInstances::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`DescribeClassicLinkInstancesOutput`](crate::output::DescribeClassicLinkInstancesOutput) with field(s):
    ///   - [`instances(Option<Vec<ClassicLinkInstance>>)`](crate::output::DescribeClassicLinkInstancesOutput::instances): <p>Information about one or more linked EC2-Classic instances.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeClassicLinkInstancesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeClassicLinkInstancesError>`](crate::error::DescribeClassicLinkInstancesError)
    pub fn describe_classic_link_instances(&self) -> crate::client::fluent_builders::DescribeClassicLinkInstances {
                                crate::client::fluent_builders::DescribeClassicLinkInstances::new(self.handle.clone())
                            }
}

