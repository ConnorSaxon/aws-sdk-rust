// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ReplaceTransitGatewayRoute`](crate::client::fluent_builders::ReplaceTransitGatewayRoute) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_cidr_block(impl Into<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::set_destination_cidr_block): <p>The CIDR range used for the destination match. Routing decisions are based on the most specific match.</p>
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::set_transit_gateway_route_table_id): <p>The ID of the route table.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::set_transit_gateway_attachment_id): <p>The ID of the attachment.</p>
    ///   - [`blackhole(bool)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::blackhole) / [`set_blackhole(Option<bool>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::set_blackhole): <p>Indicates whether traffic matching this route is to be dropped.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ReplaceTransitGatewayRoute::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`ReplaceTransitGatewayRouteOutput`](crate::output::ReplaceTransitGatewayRouteOutput) with field(s):
    ///   - [`route(Option<TransitGatewayRoute>)`](crate::output::ReplaceTransitGatewayRouteOutput::route): <p>Information about the modified route.</p>
                            /// - On failure, responds with [`SdkError<ReplaceTransitGatewayRouteError>`](crate::error::ReplaceTransitGatewayRouteError)
    pub fn replace_transit_gateway_route(&self) -> crate::client::fluent_builders::ReplaceTransitGatewayRoute {
                                crate::client::fluent_builders::ReplaceTransitGatewayRoute::new(self.handle.clone())
                            }
}

