// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTransitGatewayPolicyTable`](crate::client::fluent_builders::DeleteTransitGatewayPolicyTable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transit_gateway_policy_table_id(impl Into<String>)`](crate::client::fluent_builders::DeleteTransitGatewayPolicyTable::transit_gateway_policy_table_id) / [`set_transit_gateway_policy_table_id(Option<String>)`](crate::client::fluent_builders::DeleteTransitGatewayPolicyTable::set_transit_gateway_policy_table_id): <p>The transit gateway policy table to delete.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeleteTransitGatewayPolicyTable::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeleteTransitGatewayPolicyTable::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DeleteTransitGatewayPolicyTableOutput`](crate::output::DeleteTransitGatewayPolicyTableOutput) with field(s):
    ///   - [`transit_gateway_policy_table(Option<TransitGatewayPolicyTable>)`](crate::output::DeleteTransitGatewayPolicyTableOutput::transit_gateway_policy_table): <p>Provides details about the deleted transit gateway policy table.</p>
                            /// - On failure, responds with [`SdkError<DeleteTransitGatewayPolicyTableError>`](crate::error::DeleteTransitGatewayPolicyTableError)
    pub fn delete_transit_gateway_policy_table(&self) -> crate::client::fluent_builders::DeleteTransitGatewayPolicyTable {
                                crate::client::fluent_builders::DeleteTransitGatewayPolicyTable::new(self.handle.clone())
                            }
}

