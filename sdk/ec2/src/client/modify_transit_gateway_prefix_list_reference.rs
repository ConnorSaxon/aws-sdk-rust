// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyTransitGatewayPrefixListReference`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::set_transit_gateway_route_table_id): <p>The ID of the transit gateway route table.</p>
    ///   - [`prefix_list_id(impl Into<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::prefix_list_id) / [`set_prefix_list_id(Option<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::set_prefix_list_id): <p>The ID of the prefix list.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::set_transit_gateway_attachment_id): <p>The ID of the attachment to which traffic is routed.</p>
    ///   - [`blackhole(bool)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::blackhole) / [`set_blackhole(Option<bool>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::set_blackhole): <p>Indicates whether to drop traffic that matches this route.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`ModifyTransitGatewayPrefixListReferenceOutput`](crate::output::ModifyTransitGatewayPrefixListReferenceOutput) with field(s):
    ///   - [`transit_gateway_prefix_list_reference(Option<TransitGatewayPrefixListReference>)`](crate::output::ModifyTransitGatewayPrefixListReferenceOutput::transit_gateway_prefix_list_reference): <p>Information about the prefix list reference.</p>
                            /// - On failure, responds with [`SdkError<ModifyTransitGatewayPrefixListReferenceError>`](crate::error::ModifyTransitGatewayPrefixListReferenceError)
    pub fn modify_transit_gateway_prefix_list_reference(&self) -> crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference {
                                crate::client::fluent_builders::ModifyTransitGatewayPrefixListReference::new(self.handle.clone())
                            }
}

