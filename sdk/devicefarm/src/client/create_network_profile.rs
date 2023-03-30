// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNetworkProfile`](crate::client::fluent_builders::CreateNetworkProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_arn(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkProfile::project_arn) / [`set_project_arn(Option<String>)`](crate::client::fluent_builders::CreateNetworkProfile::set_project_arn): <p>The Amazon Resource Name (ARN) of the project for which you want to create a network profile.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkProfile::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateNetworkProfile::set_name): <p>The name for the new network profile.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkProfile::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateNetworkProfile::set_description): <p>The description of the network profile.</p>
    ///   - [`r#type(NetworkProfileType)`](crate::client::fluent_builders::CreateNetworkProfile::type) / [`set_type(Option<NetworkProfileType>)`](crate::client::fluent_builders::CreateNetworkProfile::set_type): <p>The type of network profile to create. Valid values are listed here.</p>
    ///   - [`uplink_bandwidth_bits(i64)`](crate::client::fluent_builders::CreateNetworkProfile::uplink_bandwidth_bits) / [`set_uplink_bandwidth_bits(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_uplink_bandwidth_bits): <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    ///   - [`downlink_bandwidth_bits(i64)`](crate::client::fluent_builders::CreateNetworkProfile::downlink_bandwidth_bits) / [`set_downlink_bandwidth_bits(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_downlink_bandwidth_bits): <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    ///   - [`uplink_delay_ms(i64)`](crate::client::fluent_builders::CreateNetworkProfile::uplink_delay_ms) / [`set_uplink_delay_ms(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_uplink_delay_ms): <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    ///   - [`downlink_delay_ms(i64)`](crate::client::fluent_builders::CreateNetworkProfile::downlink_delay_ms) / [`set_downlink_delay_ms(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_downlink_delay_ms): <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    ///   - [`uplink_jitter_ms(i64)`](crate::client::fluent_builders::CreateNetworkProfile::uplink_jitter_ms) / [`set_uplink_jitter_ms(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_uplink_jitter_ms): <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    ///   - [`downlink_jitter_ms(i64)`](crate::client::fluent_builders::CreateNetworkProfile::downlink_jitter_ms) / [`set_downlink_jitter_ms(Option<i64>)`](crate::client::fluent_builders::CreateNetworkProfile::set_downlink_jitter_ms): <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    ///   - [`uplink_loss_percent(i32)`](crate::client::fluent_builders::CreateNetworkProfile::uplink_loss_percent) / [`set_uplink_loss_percent(i32)`](crate::client::fluent_builders::CreateNetworkProfile::set_uplink_loss_percent): <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    ///   - [`downlink_loss_percent(i32)`](crate::client::fluent_builders::CreateNetworkProfile::downlink_loss_percent) / [`set_downlink_loss_percent(i32)`](crate::client::fluent_builders::CreateNetworkProfile::set_downlink_loss_percent): <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
                            /// - On success, responds with [`CreateNetworkProfileOutput`](crate::output::CreateNetworkProfileOutput) with field(s):
    ///   - [`network_profile(Option<NetworkProfile>)`](crate::output::CreateNetworkProfileOutput::network_profile): <p>The network profile that is returned by the create network profile request.</p>
                            /// - On failure, responds with [`SdkError<CreateNetworkProfileError>`](crate::error::CreateNetworkProfileError)
    pub fn create_network_profile(&self) -> crate::client::fluent_builders::CreateNetworkProfile {
                                crate::client::fluent_builders::CreateNetworkProfile::new(self.handle.clone())
                            }
}

