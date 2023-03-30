// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateGatewayCapabilityConfiguration`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_id(impl Into<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::gateway_id) / [`set_gateway_id(Option<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::set_gateway_id): <p>The ID of the gateway to be updated.</p>
    ///   - [`capability_namespace(impl Into<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::capability_namespace) / [`set_capability_namespace(Option<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::set_capability_namespace): <p>The namespace of the gateway capability configuration to be updated. For example, if you configure OPC-UA sources from the IoT SiteWise console, your OPC-UA capability configuration has the namespace <code>iotsitewise:opcuacollector:version</code>, where <code>version</code> is a number such as <code>1</code>.</p>
    ///   - [`capability_configuration(impl Into<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::capability_configuration) / [`set_capability_configuration(Option<String>)`](crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::set_capability_configuration): <p>The JSON document that defines the configuration for the gateway capability. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/configure-sources.html#configure-source-cli">Configuring data sources (CLI)</a> in the <i>IoT SiteWise User Guide</i>.</p>
                            /// - On success, responds with [`UpdateGatewayCapabilityConfigurationOutput`](crate::output::UpdateGatewayCapabilityConfigurationOutput) with field(s):
    ///   - [`capability_namespace(Option<String>)`](crate::output::UpdateGatewayCapabilityConfigurationOutput::capability_namespace): <p>The namespace of the gateway capability.</p>
    ///   - [`capability_sync_status(Option<CapabilitySyncStatus>)`](crate::output::UpdateGatewayCapabilityConfigurationOutput::capability_sync_status): <p>The synchronization status of the capability configuration. The sync status can be one of the following:</p>  <ul>   <li> <p> <code>IN_SYNC</code> – The gateway is running the capability configuration.</p> </li>   <li> <p> <code>OUT_OF_SYNC</code> – The gateway hasn't received the capability configuration.</p> </li>   <li> <p> <code>SYNC_FAILED</code> – The gateway rejected the capability configuration.</p> </li>  </ul>  <p>After you update a capability configuration, its sync status is <code>OUT_OF_SYNC</code> until the gateway receives and applies or rejects the updated configuration.</p>
                            /// - On failure, responds with [`SdkError<UpdateGatewayCapabilityConfigurationError>`](crate::error::UpdateGatewayCapabilityConfigurationError)
    pub fn update_gateway_capability_configuration(&self) -> crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration {
                                crate::client::fluent_builders::UpdateGatewayCapabilityConfiguration::new(self.handle.clone())
                            }
}

