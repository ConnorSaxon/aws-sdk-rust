// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTransitGatewayPeeringAttachment`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_id(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::transit_gateway_id) / [`set_transit_gateway_id(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_transit_gateway_id): <p>The ID of the transit gateway.</p>
    ///   - [`peer_transit_gateway_id(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::peer_transit_gateway_id) / [`set_peer_transit_gateway_id(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_peer_transit_gateway_id): <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    ///   - [`peer_account_id(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::peer_account_id) / [`set_peer_account_id(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_peer_account_id): <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    ///   - [`peer_region(impl Into<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::peer_region) / [`set_peer_region(Option<String>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_peer_region): <p>The Region where the peer transit gateway is located.</p>
    ///   - [`options(CreateTransitGatewayPeeringAttachmentRequestOptions)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::options) / [`set_options(Option<CreateTransitGatewayPeeringAttachmentRequestOptions>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_options): <p>Requests a transit gateway peering attachment.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_tag_specifications): <p>The tags to apply to the transit gateway peering attachment.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`CreateTransitGatewayPeeringAttachmentOutput`](crate::output::CreateTransitGatewayPeeringAttachmentOutput) with field(s):
    ///   - [`transit_gateway_peering_attachment(Option<TransitGatewayPeeringAttachment>)`](crate::output::CreateTransitGatewayPeeringAttachmentOutput::transit_gateway_peering_attachment): <p>The transit gateway peering attachment.</p>
                            /// - On failure, responds with [`SdkError<CreateTransitGatewayPeeringAttachmentError>`](crate::error::CreateTransitGatewayPeeringAttachmentError)
    pub fn create_transit_gateway_peering_attachment(&self) -> crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment {
                                crate::client::fluent_builders::CreateTransitGatewayPeeringAttachment::new(self.handle.clone())
                            }
}

