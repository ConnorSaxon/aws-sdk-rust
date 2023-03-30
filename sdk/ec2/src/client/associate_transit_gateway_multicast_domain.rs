// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateTransitGatewayMulticastDomain`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::set_transit_gateway_multicast_domain_id): <p>The ID of the transit gateway multicast domain.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::set_transit_gateway_attachment_id): <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::set_subnet_ids): <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`AssociateTransitGatewayMulticastDomainOutput`](crate::output::AssociateTransitGatewayMulticastDomainOutput) with field(s):
    ///   - [`associations(Option<TransitGatewayMulticastDomainAssociations>)`](crate::output::AssociateTransitGatewayMulticastDomainOutput::associations): <p>Information about the transit gateway multicast domain associations.</p>
                            /// - On failure, responds with [`SdkError<AssociateTransitGatewayMulticastDomainError>`](crate::error::AssociateTransitGatewayMulticastDomainError)
    pub fn associate_transit_gateway_multicast_domain(&self) -> crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain {
                                crate::client::fluent_builders::AssociateTransitGatewayMulticastDomain::new(self.handle.clone())
                            }
}

