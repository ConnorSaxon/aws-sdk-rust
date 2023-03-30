// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTransitGatewayRouteTableAnnouncement`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::set_transit_gateway_route_table_id): <p>The ID of the transit gateway route table.</p>
    ///   - [`peering_attachment_id(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::peering_attachment_id) / [`set_peering_attachment_id(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::set_peering_attachment_id): <p>The ID of the peering attachment.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::set_tag_specifications): <p>The tags specifications applied to the transit gateway route table announcement.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`CreateTransitGatewayRouteTableAnnouncementOutput`](crate::output::CreateTransitGatewayRouteTableAnnouncementOutput) with field(s):
    ///   - [`transit_gateway_route_table_announcement(Option<TransitGatewayRouteTableAnnouncement>)`](crate::output::CreateTransitGatewayRouteTableAnnouncementOutput::transit_gateway_route_table_announcement): <p>Provides details about the transit gateway route table announcement.</p>
                            /// - On failure, responds with [`SdkError<CreateTransitGatewayRouteTableAnnouncementError>`](crate::error::CreateTransitGatewayRouteTableAnnouncementError)
    pub fn create_transit_gateway_route_table_announcement(&self) -> crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement {
                                crate::client::fluent_builders::CreateTransitGatewayRouteTableAnnouncement::new(self.handle.clone())
                            }
}

