// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteLocalGatewayRouteTable`](crate::client::fluent_builders::DeleteLocalGatewayRouteTable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`local_gateway_route_table_id(impl Into<String>)`](crate::client::fluent_builders::DeleteLocalGatewayRouteTable::local_gateway_route_table_id) / [`set_local_gateway_route_table_id(Option<String>)`](crate::client::fluent_builders::DeleteLocalGatewayRouteTable::set_local_gateway_route_table_id): <p> The ID of the local gateway route table. </p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeleteLocalGatewayRouteTable::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeleteLocalGatewayRouteTable::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DeleteLocalGatewayRouteTableOutput`](crate::output::DeleteLocalGatewayRouteTableOutput) with field(s):
    ///   - [`local_gateway_route_table(Option<LocalGatewayRouteTable>)`](crate::output::DeleteLocalGatewayRouteTableOutput::local_gateway_route_table): <p>Information about the local gateway route table.</p>
                            /// - On failure, responds with [`SdkError<DeleteLocalGatewayRouteTableError>`](crate::error::DeleteLocalGatewayRouteTableError)
    pub fn delete_local_gateway_route_table(&self) -> crate::client::fluent_builders::DeleteLocalGatewayRouteTable {
                                crate::client::fluent_builders::DeleteLocalGatewayRouteTable::new(self.handle.clone())
                            }
}

