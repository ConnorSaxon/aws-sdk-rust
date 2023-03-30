// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWirelessDeviceStatistics`](crate::client::fluent_builders::GetWirelessDeviceStatistics) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`wireless_device_id(impl Into<String>)`](crate::client::fluent_builders::GetWirelessDeviceStatistics::wireless_device_id) / [`set_wireless_device_id(Option<String>)`](crate::client::fluent_builders::GetWirelessDeviceStatistics::set_wireless_device_id): <p>The ID of the wireless device for which to get the data.</p>
                            /// - On success, responds with [`GetWirelessDeviceStatisticsOutput`](crate::output::GetWirelessDeviceStatisticsOutput) with field(s):
    ///   - [`wireless_device_id(Option<String>)`](crate::output::GetWirelessDeviceStatisticsOutput::wireless_device_id): <p>The ID of the wireless device.</p>
    ///   - [`last_uplink_received_at(Option<String>)`](crate::output::GetWirelessDeviceStatisticsOutput::last_uplink_received_at): <p>The date and time when the most recent uplink was received.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanDeviceMetadata>)`](crate::output::GetWirelessDeviceStatisticsOutput::lo_ra_wan): <p>Information about the wireless device's operations.</p>
    ///   - [`sidewalk(Option<SidewalkDeviceMetadata>)`](crate::output::GetWirelessDeviceStatisticsOutput::sidewalk): <p>MetaData for Sidewalk device.</p>
                            /// - On failure, responds with [`SdkError<GetWirelessDeviceStatisticsError>`](crate::error::GetWirelessDeviceStatisticsError)
    pub fn get_wireless_device_statistics(&self) -> crate::client::fluent_builders::GetWirelessDeviceStatistics {
                                crate::client::fluent_builders::GetWirelessDeviceStatistics::new(self.handle.clone())
                            }
}

