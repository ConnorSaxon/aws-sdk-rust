// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLocalGatewayRouteTableVpcAssociations`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`local_gateway_route_table_vpc_association_ids(Vec<String>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::local_gateway_route_table_vpc_association_ids) / [`set_local_gateway_route_table_vpc_association_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::set_local_gateway_route_table_vpc_association_ids): <p>The IDs of the associations.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>   <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table for the association.</p> </li>   <li> <p> <code>local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>   <li> <p> <code>local-gateway-route-table-vpc-association-id</code> - The ID of the association.</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway route table for the association.</p> </li>   <li> <p> <code>state</code> - The state of the association.</p> </li>   <li> <p> <code>vpc-id</code> - The ID of the VPC.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeLocalGatewayRouteTableVpcAssociationsOutput`](crate::output::DescribeLocalGatewayRouteTableVpcAssociationsOutput) with field(s):
    ///   - [`local_gateway_route_table_vpc_associations(Option<Vec<LocalGatewayRouteTableVpcAssociation>>)`](crate::output::DescribeLocalGatewayRouteTableVpcAssociationsOutput::local_gateway_route_table_vpc_associations): <p>Information about the associations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeLocalGatewayRouteTableVpcAssociationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeLocalGatewayRouteTableVpcAssociationsError>`](crate::error::DescribeLocalGatewayRouteTableVpcAssociationsError)
    pub fn describe_local_gateway_route_table_vpc_associations(&self) -> crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations {
                                crate::client::fluent_builders::DescribeLocalGatewayRouteTableVpcAssociations::new(self.handle.clone())
                            }
}

