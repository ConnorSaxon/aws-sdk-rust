// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateNetworkAnalyzerConfiguration`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_name(impl Into<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::configuration_name) / [`set_configuration_name(Option<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_configuration_name): <p>Name of the network analyzer configuration.</p>
    ///   - [`trace_content(TraceContent)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::trace_content) / [`set_trace_content(Option<TraceContent>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_trace_content): <p>Trace content for your wireless gateway and wireless device resources.</p>
    ///   - [`wireless_devices_to_add(Vec<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::wireless_devices_to_add) / [`set_wireless_devices_to_add(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_wireless_devices_to_add): <p>Wireless device resources to add to the network analyzer configuration. Provide the <code>WirelessDeviceId</code> of the resource to add in the input array.</p>
    ///   - [`wireless_devices_to_remove(Vec<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::wireless_devices_to_remove) / [`set_wireless_devices_to_remove(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_wireless_devices_to_remove): <p>Wireless device resources to remove from the network analyzer configuration. Provide the <code>WirelessDeviceId</code> of the resources to remove in the input array.</p>
    ///   - [`wireless_gateways_to_add(Vec<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::wireless_gateways_to_add) / [`set_wireless_gateways_to_add(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_wireless_gateways_to_add): <p>Wireless gateway resources to add to the network analyzer configuration. Provide the <code>WirelessGatewayId</code> of the resource to add in the input array.</p>
    ///   - [`wireless_gateways_to_remove(Vec<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::wireless_gateways_to_remove) / [`set_wireless_gateways_to_remove(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_wireless_gateways_to_remove): <p>Wireless gateway resources to remove from the network analyzer configuration. Provide the <code>WirelessGatewayId</code> of the resources to remove in the input array.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::set_description): <p>The description of the new resource.</p>
                            /// - On success, responds with [`UpdateNetworkAnalyzerConfigurationOutput`](crate::output::UpdateNetworkAnalyzerConfigurationOutput)
                            /// - On failure, responds with [`SdkError<UpdateNetworkAnalyzerConfigurationError>`](crate::error::UpdateNetworkAnalyzerConfigurationError)
    pub fn update_network_analyzer_configuration(&self) -> crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration {
                                crate::client::fluent_builders::UpdateNetworkAnalyzerConfiguration::new(self.handle.clone())
                            }
}

