// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AllocateHostedConnection`](crate::client::fluent_builders::AllocateHostedConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`connection_id(impl Into<String>)`](crate::client::fluent_builders::AllocateHostedConnection::connection_id) / [`set_connection_id(Option<String>)`](crate::client::fluent_builders::AllocateHostedConnection::set_connection_id): <p>The ID of the interconnect or LAG.</p>
    ///   - [`owner_account(impl Into<String>)`](crate::client::fluent_builders::AllocateHostedConnection::owner_account) / [`set_owner_account(Option<String>)`](crate::client::fluent_builders::AllocateHostedConnection::set_owner_account): <p>The ID of the Amazon Web Services account ID of the customer for the connection.</p>
    ///   - [`bandwidth(impl Into<String>)`](crate::client::fluent_builders::AllocateHostedConnection::bandwidth) / [`set_bandwidth(Option<String>)`](crate::client::fluent_builders::AllocateHostedConnection::set_bandwidth): <p>The bandwidth of the connection. The possible values are 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, and 10Gbps. Note that only those Direct Connect Partners who have met specific requirements are allowed to create a 1Gbps, 2Gbps, 5Gbps or 10Gbps hosted connection. </p>
    ///   - [`connection_name(impl Into<String>)`](crate::client::fluent_builders::AllocateHostedConnection::connection_name) / [`set_connection_name(Option<String>)`](crate::client::fluent_builders::AllocateHostedConnection::set_connection_name): <p>The name of the hosted connection.</p>
    ///   - [`vlan(i32)`](crate::client::fluent_builders::AllocateHostedConnection::vlan) / [`set_vlan(i32)`](crate::client::fluent_builders::AllocateHostedConnection::set_vlan): <p>The dedicated VLAN provisioned to the hosted connection.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::AllocateHostedConnection::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::AllocateHostedConnection::set_tags): <p>The tags associated with the connection.</p>
                            /// - On success, responds with [`AllocateHostedConnectionOutput`](crate::output::AllocateHostedConnectionOutput) with field(s):
    ///   - [`owner_account(Option<String>)`](crate::output::AllocateHostedConnectionOutput::owner_account): <p>The ID of the Amazon Web Services account that owns the connection.</p>
    ///   - [`connection_id(Option<String>)`](crate::output::AllocateHostedConnectionOutput::connection_id): <p>The ID of the connection.</p>
    ///   - [`connection_name(Option<String>)`](crate::output::AllocateHostedConnectionOutput::connection_name): <p>The name of the connection.</p>
    ///   - [`connection_state(Option<ConnectionState>)`](crate::output::AllocateHostedConnectionOutput::connection_state): <p>The state of the connection. The following are the possible values:</p>  <ul>   <li> <p> <code>ordering</code>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li>   <li> <p> <code>requested</code>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li>   <li> <p> <code>pending</code>: The connection has been approved and is being initialized.</p> </li>   <li> <p> <code>available</code>: The network link is up and the connection is ready for use.</p> </li>   <li> <p> <code>down</code>: The network link is down.</p> </li>   <li> <p> <code>deleting</code>: The connection is being deleted.</p> </li>   <li> <p> <code>deleted</code>: The connection has been deleted.</p> </li>   <li> <p> <code>rejected</code>: A hosted connection in the <code>ordering</code> state enters the <code>rejected</code> state if it is deleted by the customer.</p> </li>   <li> <p> <code>unknown</code>: The state of the connection is not available.</p> </li>  </ul>
    ///   - [`region(Option<String>)`](crate::output::AllocateHostedConnectionOutput::region): <p>The Amazon Web Services Region where the connection is located.</p>
    ///   - [`location(Option<String>)`](crate::output::AllocateHostedConnectionOutput::location): <p>The location of the connection.</p>
    ///   - [`bandwidth(Option<String>)`](crate::output::AllocateHostedConnectionOutput::bandwidth): <p>The bandwidth of the connection.</p>
    ///   - [`vlan(i32)`](crate::output::AllocateHostedConnectionOutput::vlan): <p>The ID of the VLAN.</p>
    ///   - [`partner_name(Option<String>)`](crate::output::AllocateHostedConnectionOutput::partner_name): <p>The name of the Direct Connect service provider associated with the connection.</p>
    ///   - [`loa_issue_time(Option<DateTime>)`](crate::output::AllocateHostedConnectionOutput::loa_issue_time): <p>The time of the most recent call to <code>DescribeLoa</code> for this connection.</p>
    ///   - [`lag_id(Option<String>)`](crate::output::AllocateHostedConnectionOutput::lag_id): <p>The ID of the LAG.</p>
    ///   - [`aws_device(Option<String>)`](crate::output::AllocateHostedConnectionOutput::aws_device): <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    ///   - [`jumbo_frame_capable(Option<bool>)`](crate::output::AllocateHostedConnectionOutput::jumbo_frame_capable): <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    ///   - [`aws_device_v2(Option<String>)`](crate::output::AllocateHostedConnectionOutput::aws_device_v2): <p>The Direct Connect endpoint that terminates the physical connection.</p>
    ///   - [`aws_logical_device_id(Option<String>)`](crate::output::AllocateHostedConnectionOutput::aws_logical_device_id): <p>The Direct Connect endpoint that terminates the logical connection. This device might be different than the device that terminates the physical connection.</p>
    ///   - [`has_logical_redundancy(Option<HasLogicalRedundancy>)`](crate::output::AllocateHostedConnectionOutput::has_logical_redundancy): <p>Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::AllocateHostedConnectionOutput::tags): <p>The tags associated with the connection.</p>
    ///   - [`provider_name(Option<String>)`](crate::output::AllocateHostedConnectionOutput::provider_name): <p>The name of the service provider associated with the connection.</p>
    ///   - [`mac_sec_capable(Option<bool>)`](crate::output::AllocateHostedConnectionOutput::mac_sec_capable): <p>Indicates whether the connection supports MAC Security (MACsec).</p>
    ///   - [`port_encryption_status(Option<String>)`](crate::output::AllocateHostedConnectionOutput::port_encryption_status): <p>The MAC Security (MACsec) port link status of the connection.</p>  <p>The valid values are <code>Encryption Up</code>, which means that there is an active Connection Key Name, or <code>Encryption Down</code>.</p>
    ///   - [`encryption_mode(Option<String>)`](crate::output::AllocateHostedConnectionOutput::encryption_mode): <p>The MAC Security (MACsec) connection encryption mode.</p>  <p>The valid values are <code>no_encrypt</code>, <code>should_encrypt</code>, and <code>must_encrypt</code>.</p>
    ///   - [`mac_sec_keys(Option<Vec<MacSecKey>>)`](crate::output::AllocateHostedConnectionOutput::mac_sec_keys): <p>The MAC Security (MACsec) security keys associated with the connection.</p>
                            /// - On failure, responds with [`SdkError<AllocateHostedConnectionError>`](crate::error::AllocateHostedConnectionError)
    pub fn allocate_hosted_connection(&self) -> crate::client::fluent_builders::AllocateHostedConnection {
                                crate::client::fluent_builders::AllocateHostedConnection::new(self.handle.clone())
                            }
}

