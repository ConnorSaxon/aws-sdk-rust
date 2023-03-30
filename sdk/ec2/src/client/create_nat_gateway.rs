// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNatGateway`](crate::client::fluent_builders::CreateNatGateway) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`allocation_id(impl Into<String>)`](crate::client::fluent_builders::CreateNatGateway::allocation_id) / [`set_allocation_id(Option<String>)`](crate::client::fluent_builders::CreateNatGateway::set_allocation_id): <p>[Public NAT gateways only] The allocation ID of an Elastic IP address to associate with the NAT gateway. You cannot specify an Elastic IP address with a private NAT gateway. If the Elastic IP address is associated with another resource, you must first disassociate it.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateNatGateway::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateNatGateway::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>  <p>Constraint: Maximum 64 ASCII characters.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateNatGateway::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateNatGateway::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`subnet_id(impl Into<String>)`](crate::client::fluent_builders::CreateNatGateway::subnet_id) / [`set_subnet_id(Option<String>)`](crate::client::fluent_builders::CreateNatGateway::set_subnet_id): <p>The subnet in which to create the NAT gateway.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateNatGateway::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateNatGateway::set_tag_specifications): <p>The tags to assign to the NAT gateway.</p>
    ///   - [`connectivity_type(ConnectivityType)`](crate::client::fluent_builders::CreateNatGateway::connectivity_type) / [`set_connectivity_type(Option<ConnectivityType>)`](crate::client::fluent_builders::CreateNatGateway::set_connectivity_type): <p>Indicates whether the NAT gateway supports public or private connectivity. The default is public connectivity.</p>
    ///   - [`private_ip_address(impl Into<String>)`](crate::client::fluent_builders::CreateNatGateway::private_ip_address) / [`set_private_ip_address(Option<String>)`](crate::client::fluent_builders::CreateNatGateway::set_private_ip_address): <p>The private IPv4 address to assign to the NAT gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.</p>
                            /// - On success, responds with [`CreateNatGatewayOutput`](crate::output::CreateNatGatewayOutput) with field(s):
    ///   - [`client_token(Option<String>)`](crate::output::CreateNatGatewayOutput::client_token): <p>Unique, case-sensitive identifier to ensure the idempotency of the request. Only returned if a client token was provided in the request.</p>
    ///   - [`nat_gateway(Option<NatGateway>)`](crate::output::CreateNatGatewayOutput::nat_gateway): <p>Information about the NAT gateway.</p>
                            /// - On failure, responds with [`SdkError<CreateNatGatewayError>`](crate::error::CreateNatGatewayError)
    pub fn create_nat_gateway(&self) -> crate::client::fluent_builders::CreateNatGateway {
                                crate::client::fluent_builders::CreateNatGateway::new(self.handle.clone())
                            }
}

