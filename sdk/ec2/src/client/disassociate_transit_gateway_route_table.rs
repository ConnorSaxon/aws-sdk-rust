// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateTransitGatewayRouteTable`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::set_transit_gateway_route_table_id): <p>The ID of the transit gateway route table.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::set_transit_gateway_attachment_id): <p>The ID of the attachment.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DisassociateTransitGatewayRouteTableOutput`](crate::output::DisassociateTransitGatewayRouteTableOutput) with field(s):
    ///   - [`association(Option<TransitGatewayAssociation>)`](crate::output::DisassociateTransitGatewayRouteTableOutput::association): <p>Information about the association.</p>
                            /// - On failure, responds with [`SdkError<DisassociateTransitGatewayRouteTableError>`](crate::error::DisassociateTransitGatewayRouteTableError)
    pub fn disassociate_transit_gateway_route_table(&self) -> crate::client::fluent_builders::DisassociateTransitGatewayRouteTable {
                                crate::client::fluent_builders::DisassociateTransitGatewayRouteTable::new(self.handle.clone())
                            }
}

