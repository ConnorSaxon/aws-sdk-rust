// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetServiceSettings`](crate::client::fluent_builders::GetServiceSettings) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetServiceSettings::send) it.
                            /// - On success, responds with [`GetServiceSettingsOutput`](crate::output::GetServiceSettingsOutput) with field(s):
    ///   - [`linux_subscriptions_discovery(Option<LinuxSubscriptionsDiscovery>)`](crate::output::GetServiceSettingsOutput::linux_subscriptions_discovery): <p>Lists if discovery has been enabled for Linux subscriptions.</p>
    ///   - [`linux_subscriptions_discovery_settings(Option<LinuxSubscriptionsDiscoverySettings>)`](crate::output::GetServiceSettingsOutput::linux_subscriptions_discovery_settings): <p>Lists the settings defined for Linux subscriptions discovery. The settings include if Organizations integration has been enabled, and which Regions data will be aggregated from.</p>
    ///   - [`status(Option<Status>)`](crate::output::GetServiceSettingsOutput::status): <p>Indicates the status of Linux subscriptions settings being applied.</p>
    ///   - [`status_message(Option<HashMap<String, String>>)`](crate::output::GetServiceSettingsOutput::status_message): <p>A message which details the Linux subscriptions service settings current status.</p>
    ///   - [`home_regions(Option<Vec<String>>)`](crate::output::GetServiceSettingsOutput::home_regions): <p>The Region in which License Manager displays the aggregated data for Linux subscriptions.</p>
                            /// - On failure, responds with [`SdkError<GetServiceSettingsError>`](crate::error::GetServiceSettingsError)
    pub fn get_service_settings(&self) -> crate::client::fluent_builders::GetServiceSettings {
                                crate::client::fluent_builders::GetServiceSettings::new(self.handle.clone())
                            }
}

