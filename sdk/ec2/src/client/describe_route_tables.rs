// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeRouteTables`](crate::client::fluent_builders::DescribeRouteTables) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeRouteTables::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeRouteTables::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeRouteTables::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>association.route-table-association-id</code> - The ID of an association ID for the route table.</p> </li>   <li> <p> <code>association.route-table-id</code> - The ID of the route table involved in the association.</p> </li>   <li> <p> <code>association.subnet-id</code> - The ID of the subnet involved in the association.</p> </li>   <li> <p> <code>association.main</code> - Indicates whether the route table is the main route table for the VPC (<code>true</code> | <code>false</code>). Route tables that do not have an association ID are not returned in the response.</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the route table.</p> </li>   <li> <p> <code>route-table-id</code> - The ID of the route table.</p> </li>   <li> <p> <code>route.destination-cidr-block</code> - The IPv4 CIDR range specified in a route in the table.</p> </li>   <li> <p> <code>route.destination-ipv6-cidr-block</code> - The IPv6 CIDR range specified in a route in the route table.</p> </li>   <li> <p> <code>route.destination-prefix-list-id</code> - The ID (prefix) of the Amazon Web Service specified in a route in the table.</p> </li>   <li> <p> <code>route.egress-only-internet-gateway-id</code> - The ID of an egress-only Internet gateway specified in a route in the route table.</p> </li>   <li> <p> <code>route.gateway-id</code> - The ID of a gateway specified in a route in the table.</p> </li>   <li> <p> <code>route.instance-id</code> - The ID of an instance specified in a route in the table.</p> </li>   <li> <p> <code>route.nat-gateway-id</code> - The ID of a NAT gateway.</p> </li>   <li> <p> <code>route.transit-gateway-id</code> - The ID of a transit gateway.</p> </li>   <li> <p> <code>route.origin</code> - Describes how the route was created. <code>CreateRouteTable</code> indicates that the route was automatically created when the route table was created; <code>CreateRoute</code> indicates that the route was manually added to the route table; <code>EnableVgwRoutePropagation</code> indicates that the route was propagated by route propagation.</p> </li>   <li> <p> <code>route.state</code> - The state of a route in the route table (<code>active</code> | <code>blackhole</code>). The blackhole state indicates that the route's target isn't available (for example, the specified gateway isn't attached to the VPC, the specified NAT instance has been terminated, and so on).</p> </li>   <li> <p> <code>route.vpc-peering-connection-id</code> - The ID of a VPC peering connection specified in a route in the table.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>vpc-id</code> - The ID of the VPC for the route table.</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeRouteTables::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeRouteTables::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`route_table_ids(Vec<String>)`](crate::client::fluent_builders::DescribeRouteTables::route_table_ids) / [`set_route_table_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeRouteTables::set_route_table_ids): <p>One or more route table IDs.</p>  <p>Default: Describes all your route tables.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeRouteTables::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeRouteTables::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeRouteTables::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeRouteTables::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
                            /// - On success, responds with [`DescribeRouteTablesOutput`](crate::output::DescribeRouteTablesOutput) with field(s):
    ///   - [`route_tables(Option<Vec<RouteTable>>)`](crate::output::DescribeRouteTablesOutput::route_tables): <p>Information about one or more route tables.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeRouteTablesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeRouteTablesError>`](crate::error::DescribeRouteTablesError)
    pub fn describe_route_tables(&self) -> crate::client::fluent_builders::DescribeRouteTables {
                                crate::client::fluent_builders::DescribeRouteTables::new(self.handle.clone())
                            }
}

