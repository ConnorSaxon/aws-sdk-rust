// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterTransitGatewayMulticastGroupSources`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::set_transit_gateway_multicast_domain_id): <p>The ID of the transit gateway multicast domain.</p>
    ///   - [`group_ip_address(impl Into<String>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::group_ip_address) / [`set_group_ip_address(Option<String>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::set_group_ip_address): <p>The IP address assigned to the transit gateway multicast group.</p>
    ///   - [`network_interface_ids(Vec<String>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::network_interface_ids) / [`set_network_interface_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::set_network_interface_ids): <p>The IDs of the group sources' network interfaces.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DeregisterTransitGatewayMulticastGroupSourcesOutput`](crate::output::DeregisterTransitGatewayMulticastGroupSourcesOutput) with field(s):
    ///   - [`deregistered_multicast_group_sources(Option<TransitGatewayMulticastDeregisteredGroupSources>)`](crate::output::DeregisterTransitGatewayMulticastGroupSourcesOutput::deregistered_multicast_group_sources): <p>Information about the deregistered group sources.</p>
                            /// - On failure, responds with [`SdkError<DeregisterTransitGatewayMulticastGroupSourcesError>`](crate::error::DeregisterTransitGatewayMulticastGroupSourcesError)
    pub fn deregister_transit_gateway_multicast_group_sources(&self) -> crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources {
                                crate::client::fluent_builders::DeregisterTransitGatewayMulticastGroupSources::new(self.handle.clone())
                            }
}

