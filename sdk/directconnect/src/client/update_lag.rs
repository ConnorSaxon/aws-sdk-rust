// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateLag`](crate::client::fluent_builders::UpdateLag) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`lag_id(impl Into<String>)`](crate::client::fluent_builders::UpdateLag::lag_id) / [`set_lag_id(Option<String>)`](crate::client::fluent_builders::UpdateLag::set_lag_id): <p>The ID of the LAG.</p>
    ///   - [`lag_name(impl Into<String>)`](crate::client::fluent_builders::UpdateLag::lag_name) / [`set_lag_name(Option<String>)`](crate::client::fluent_builders::UpdateLag::set_lag_name): <p>The name of the LAG.</p>
    ///   - [`minimum_links(i32)`](crate::client::fluent_builders::UpdateLag::minimum_links) / [`set_minimum_links(i32)`](crate::client::fluent_builders::UpdateLag::set_minimum_links): <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    ///   - [`encryption_mode(impl Into<String>)`](crate::client::fluent_builders::UpdateLag::encryption_mode) / [`set_encryption_mode(Option<String>)`](crate::client::fluent_builders::UpdateLag::set_encryption_mode): <p>The LAG MAC Security (MACsec) encryption mode.</p>  <p>Amazon Web Services applies the value to all connections which are part of the LAG.</p>
                            /// - On success, responds with [`UpdateLagOutput`](crate::output::UpdateLagOutput) with field(s):
    ///   - [`connections_bandwidth(Option<String>)`](crate::output::UpdateLagOutput::connections_bandwidth): <p>The individual bandwidth of the physical connections bundled by the LAG. The possible values are 1Gbps and 10Gbps. </p>
    ///   - [`number_of_connections(i32)`](crate::output::UpdateLagOutput::number_of_connections): <p>The number of physical dedicated connections bundled by the LAG, up to a maximum of 10.</p>
    ///   - [`lag_id(Option<String>)`](crate::output::UpdateLagOutput::lag_id): <p>The ID of the LAG.</p>
    ///   - [`owner_account(Option<String>)`](crate::output::UpdateLagOutput::owner_account): <p>The ID of the Amazon Web Services account that owns the LAG.</p>
    ///   - [`lag_name(Option<String>)`](crate::output::UpdateLagOutput::lag_name): <p>The name of the LAG.</p>
    ///   - [`lag_state(Option<LagState>)`](crate::output::UpdateLagOutput::lag_state): <p>The state of the LAG. The following are the possible values:</p>  <ul>   <li> <p> <code>requested</code>: The initial state of a LAG. The LAG stays in the requested state until the Letter of Authorization (LOA) is available.</p> </li>   <li> <p> <code>pending</code>: The LAG has been approved and is being initialized.</p> </li>   <li> <p> <code>available</code>: The network link is established and the LAG is ready for use.</p> </li>   <li> <p> <code>down</code>: The network link is down.</p> </li>   <li> <p> <code>deleting</code>: The LAG is being deleted.</p> </li>   <li> <p> <code>deleted</code>: The LAG is deleted.</p> </li>   <li> <p> <code>unknown</code>: The state of the LAG is not available.</p> </li>  </ul>
    ///   - [`location(Option<String>)`](crate::output::UpdateLagOutput::location): <p>The location of the LAG.</p>
    ///   - [`region(Option<String>)`](crate::output::UpdateLagOutput::region): <p>The Amazon Web Services Region where the connection is located.</p>
    ///   - [`minimum_links(i32)`](crate::output::UpdateLagOutput::minimum_links): <p>The minimum number of physical dedicated connections that must be operational for the LAG itself to be operational.</p>
    ///   - [`aws_device(Option<String>)`](crate::output::UpdateLagOutput::aws_device): <p>The Direct Connect endpoint that hosts the LAG.</p>
    ///   - [`aws_device_v2(Option<String>)`](crate::output::UpdateLagOutput::aws_device_v2): <p>The Direct Connect endpoint that hosts the LAG.</p>
    ///   - [`aws_logical_device_id(Option<String>)`](crate::output::UpdateLagOutput::aws_logical_device_id): <p>The Direct Connect endpoint that terminates the logical connection. This device might be different than the device that terminates the physical connection.</p>
    ///   - [`connections(Option<Vec<Connection>>)`](crate::output::UpdateLagOutput::connections): <p>The connections bundled by the LAG.</p>
    ///   - [`allows_hosted_connections(bool)`](crate::output::UpdateLagOutput::allows_hosted_connections): <p>Indicates whether the LAG can host other connections.</p>
    ///   - [`jumbo_frame_capable(Option<bool>)`](crate::output::UpdateLagOutput::jumbo_frame_capable): <p>Indicates whether jumbo frames (9001 MTU) are supported.</p>
    ///   - [`has_logical_redundancy(Option<HasLogicalRedundancy>)`](crate::output::UpdateLagOutput::has_logical_redundancy): <p>Indicates whether the LAG supports a secondary BGP peer in the same address family (IPv4/IPv6).</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::UpdateLagOutput::tags): <p>The tags associated with the LAG.</p>
    ///   - [`provider_name(Option<String>)`](crate::output::UpdateLagOutput::provider_name): <p>The name of the service provider associated with the LAG.</p>
    ///   - [`mac_sec_capable(Option<bool>)`](crate::output::UpdateLagOutput::mac_sec_capable): <p>Indicates whether the LAG supports MAC Security (MACsec).</p>
    ///   - [`encryption_mode(Option<String>)`](crate::output::UpdateLagOutput::encryption_mode): <p>The LAG MAC Security (MACsec) encryption mode.</p>  <p>The valid values are <code>no_encrypt</code>, <code>should_encrypt</code>, and <code>must_encrypt</code>.</p>
    ///   - [`mac_sec_keys(Option<Vec<MacSecKey>>)`](crate::output::UpdateLagOutput::mac_sec_keys): <p>The MAC Security (MACsec) security keys associated with the LAG.</p>
                            /// - On failure, responds with [`SdkError<UpdateLagError>`](crate::error::UpdateLagError)
    pub fn update_lag(&self) -> crate::client::fluent_builders::UpdateLag {
                                crate::client::fluent_builders::UpdateLag::new(self.handle.clone())
                            }
}

