// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateConnection`](crate::client::fluent_builders::CreateConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`location(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::location) / [`set_location(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_location): <p>The location of the connection.</p>
    ///   - [`bandwidth(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::bandwidth) / [`set_bandwidth(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_bandwidth): <p>The bandwidth of the connection.</p>
    ///   - [`connection_name(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::connection_name) / [`set_connection_name(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_connection_name): <p>The name of the connection.</p>
    ///   - [`lag_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::lag_id) / [`set_lag_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_lag_id): <p>The ID of the LAG.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateConnection::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateConnection::set_tags): <p>The tags to associate with the lag.</p>
    ///   - [`provider_name(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::provider_name) / [`set_provider_name(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_provider_name): <p>The name of the service provider associated with the requested connection.</p>
    ///   - [`request_mac_sec(bool)`](crate::client::fluent_builders::CreateConnection::request_mac_sec) / [`set_request_mac_sec(Option<bool>)`](crate::client::fluent_builders::CreateConnection::set_request_mac_sec): <p>Indicates whether you want the connection to support MAC Security (MACsec).</p>  <p>MAC Security (MACsec) is only available on dedicated connections. For information about MAC Security (MACsec) prerequisties, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites">MACsec prerequisties</a> in the <i>Direct Connect User Guide</i>.</p>
                            /// - On success, responds with [`CreateConnectionOutput`](crate::output::CreateConnectionOutput) with field(s):
    ///   - [`owner_account(Option<String>)`](crate::output::CreateConnectionOutput::owner_account): <p>The ID of the Amazon Web Services account that owns the connection.</p>
    ///   - [`connection_id(Option<String>)`](crate::output::CreateConnectionOutput::connection_id): <p>The ID of the connection.</p>
    ///   - [`connection_name(Option<String>)`](crate::output::CreateConnectionOutput::connection_name): <p>The name of the connection.</p>
    ///   - [`connection_state(Option<ConnectionState>)`](crate::output::CreateConnectionOutput::connection_state): <p>The state of the connection. The following are the possible values:</p>  <ul>   <li> <p> <code>ordering</code>: The initial state of a hosted connection provisioned on an interconnect. The connection stays in the ordering state until the owner of the hosted connection confirms or declines the connection order.</p> </li>   <li> <p> <code>requested</code>: The initial state of a standard connection. The connection stays in the requested state until the Letter of Authorization (LOA) is sent to the customer.</p> </li>   <li> <p> <code>pending</code>: The connection has been approved and is being initialized.</p> </li>   <li> <p> <code>available</code>: The network link is up and the connection is ready for use.</p> </li>   <li> <p> <code>down</code>: The network link is down.</p> </li>   <li> <p> <code>deleting</code>: The connection is being deleted.</p> </li>   <li> <p> <code>deleted</code>: The connection has been deleted.</p> </li>   <li> <p> <code>rejected</code>: A hosted connection in the <code>ordering</code> state enters the <code>rejected</code> state if it is deleted by the customer.</p> </li>   <li> <p> <code>unknown</code>: The state of the connection is not available.</p> </li>  </ul>
    ///   - [`region(Option<String>)`](crate::output::CreateConnectionOutput::region): <p>The Amazon Web Services Region where the connection is located.</p>
    ///   - [`location(Option<String>)`](crate::output::CreateConnectionOutput::location): <p>The location of the connection.</p>
    ///   - [`bandwidth(Option<String>)`](crate::output::CreateConnectionOutput::bandwidth): <p>The bandwidth of the connection.</p>
    ///   - [`vlan(i32)`](crate::output::CreateConnectionOutput::vlan): <p>The ID of the VLAN.</p>
    ///   - [`partner_name(Option<String>)`](crate::output::CreateConnectionOutput::partner_name): <p>The name of the Direct Connect service provider associated with the connection.</p>
    ///   - [`loa_issue_time(Option<DateTime>)`](crate::output::CreateConnectionOutput::loa_issue_time): <p>The time of the most recent call to <code>DescribeLoa</code> for this connection.</p>
    ///   - [`lag_id(Option<String>)`](crate::output::CreateConnectionOutput::lag_id): <p>The ID of the LAG.</p>
    ///   - [`aws_device(Option<String>)`](crate::output::CreateConnectionOutput::aws_device): <p>The Direct Connect endpoint on which the physical connection terminates.</p>
    ///   - [`jumbo_frame_capable(Option<bool>)`](crate::output::CreateConnectionOutput::jumbo_frame_capable): <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    ///   - [`aws_device_v2(Option<String>)`](crate::output::CreateConnectionOutput::aws_device_v2): <p>The Direct Connect endpoint that terminates the physical connection.</p>
    ///   - [`aws_logical_device_id(Option<String>)`](crate::output::CreateConnectionOutput::aws_logical_device_id): <p>The Direct Connect endpoint that terminates the logical connection. This device might be different than the device that terminates the physical connection.</p>
    ///   - [`has_logical_redundancy(Option<HasLogicalRedundancy>)`](crate::output::CreateConnectionOutput::has_logical_redundancy): <p>Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::CreateConnectionOutput::tags): <p>The tags associated with the connection.</p>
    ///   - [`provider_name(Option<String>)`](crate::output::CreateConnectionOutput::provider_name): <p>The name of the service provider associated with the connection.</p>
    ///   - [`mac_sec_capable(Option<bool>)`](crate::output::CreateConnectionOutput::mac_sec_capable): <p>Indicates whether the connection supports MAC Security (MACsec).</p>
    ///   - [`port_encryption_status(Option<String>)`](crate::output::CreateConnectionOutput::port_encryption_status): <p>The MAC Security (MACsec) port link status of the connection.</p>  <p>The valid values are <code>Encryption Up</code>, which means that there is an active Connection Key Name, or <code>Encryption Down</code>.</p>
    ///   - [`encryption_mode(Option<String>)`](crate::output::CreateConnectionOutput::encryption_mode): <p>The MAC Security (MACsec) connection encryption mode.</p>  <p>The valid values are <code>no_encrypt</code>, <code>should_encrypt</code>, and <code>must_encrypt</code>.</p>
    ///   - [`mac_sec_keys(Option<Vec<MacSecKey>>)`](crate::output::CreateConnectionOutput::mac_sec_keys): <p>The MAC Security (MACsec) security keys associated with the connection.</p>
                            /// - On failure, responds with [`SdkError<CreateConnectionError>`](crate::error::CreateConnectionError)
    pub fn create_connection(&self) -> crate::client::fluent_builders::CreateConnection {
                                crate::client::fluent_builders::CreateConnection::new(self.handle.clone())
                            }
}

