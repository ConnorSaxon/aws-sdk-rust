// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcceptVpcPeeringConnection`](crate::client::fluent_builders::AcceptVpcPeeringConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AcceptVpcPeeringConnection::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AcceptVpcPeeringConnection::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`vpc_peering_connection_id(impl Into<String>)`](crate::client::fluent_builders::AcceptVpcPeeringConnection::vpc_peering_connection_id) / [`set_vpc_peering_connection_id(Option<String>)`](crate::client::fluent_builders::AcceptVpcPeeringConnection::set_vpc_peering_connection_id): <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
                            /// - On success, responds with [`AcceptVpcPeeringConnectionOutput`](crate::output::AcceptVpcPeeringConnectionOutput) with field(s):
    ///   - [`vpc_peering_connection(Option<VpcPeeringConnection>)`](crate::output::AcceptVpcPeeringConnectionOutput::vpc_peering_connection): <p>Information about the VPC peering connection.</p>
                            /// - On failure, responds with [`SdkError<AcceptVpcPeeringConnectionError>`](crate::error::AcceptVpcPeeringConnectionError)
    pub fn accept_vpc_peering_connection(&self) -> crate::client::fluent_builders::AcceptVpcPeeringConnection {
                                crate::client::fluent_builders::AcceptVpcPeeringConnection::new(self.handle.clone())
                            }
}

