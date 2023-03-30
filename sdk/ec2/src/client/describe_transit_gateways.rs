// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeTransitGateways`](crate::client::fluent_builders::DescribeTransitGateways) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeTransitGateways::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_ids(Vec<String>)`](crate::client::fluent_builders::DescribeTransitGateways::transit_gateway_ids) / [`set_transit_gateway_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeTransitGateways::set_transit_gateway_ids): <p>The IDs of the transit gateways.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeTransitGateways::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeTransitGateways::set_filters): <p>One or more filters. The possible values are:</p>  <ul>   <li> <p> <code>options.propagation-default-route-table-id</code> - The ID of the default propagation route table.</p> </li>   <li> <p> <code>options.amazon-side-asn</code> - The private ASN for the Amazon side of a BGP session.</p> </li>   <li> <p> <code>options.association-default-route-table-id</code> - The ID of the default association route table.</p> </li>   <li> <p> <code>options.auto-accept-shared-attachments</code> - Indicates whether there is automatic acceptance of attachment requests (<code>enable</code> | <code>disable</code>).</p> </li>   <li> <p> <code>options.default-route-table-association</code> - Indicates whether resource attachments are automatically associated with the default association route table (<code>enable</code> | <code>disable</code>).</p> </li>   <li> <p> <code>options.default-route-table-propagation</code> - Indicates whether resource attachments automatically propagate routes to the default propagation route table (<code>enable</code> | <code>disable</code>).</p> </li>   <li> <p> <code>options.dns-support</code> - Indicates whether DNS support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>   <li> <p> <code>options.vpn-ecmp-support</code> - Indicates whether Equal Cost Multipath Protocol support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the transit gateway.</p> </li>   <li> <p> <code>state</code> - The state of the transit gateway (<code>available</code> | <code>deleted</code> | <code>deleting</code> | <code>modifying</code> | <code>pending</code>).</p> </li>   <li> <p> <code>transit-gateway-id</code> - The ID of the transit gateway.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeTransitGateways::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeTransitGateways::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeTransitGateways::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeTransitGateways::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeTransitGateways::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeTransitGateways::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeTransitGatewaysOutput`](crate::output::DescribeTransitGatewaysOutput) with field(s):
    ///   - [`transit_gateways(Option<Vec<TransitGateway>>)`](crate::output::DescribeTransitGatewaysOutput::transit_gateways): <p>Information about the transit gateways.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeTransitGatewaysOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeTransitGatewaysError>`](crate::error::DescribeTransitGatewaysError)
    pub fn describe_transit_gateways(&self) -> crate::client::fluent_builders::DescribeTransitGateways {
                                crate::client::fluent_builders::DescribeTransitGateways::new(self.handle.clone())
                            }
}

