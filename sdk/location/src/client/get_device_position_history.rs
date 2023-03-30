// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDevicePositionHistory`](crate::client::fluent_builders::GetDevicePositionHistory) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetDevicePositionHistory::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`tracker_name(impl Into<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::tracker_name) / [`set_tracker_name(Option<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_tracker_name): <p>The tracker resource receiving the request for the device position history.</p>
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_device_id): <p>The device whose position history you want to retrieve.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_next_token): <p>The pagination token specifying which page of results to return in the response. If no token is provided, the default page is the first page. </p>  <p>Default value: <code>null</code> </p>
    ///   - [`start_time_inclusive(DateTime)`](crate::client::fluent_builders::GetDevicePositionHistory::start_time_inclusive) / [`set_start_time_inclusive(Option<DateTime>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_start_time_inclusive): <p>Specify the start time for the position history in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. By default, the value will be 24 hours prior to the time that the request is made.</p>  <p>Requirement:</p>  <ul>   <li> <p>The time specified for <code>StartTimeInclusive</code> must be before <code>EndTimeExclusive</code>.</p> </li>  </ul>
    ///   - [`end_time_exclusive(DateTime)`](crate::client::fluent_builders::GetDevicePositionHistory::end_time_exclusive) / [`set_end_time_exclusive(Option<DateTime>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_end_time_exclusive): <p>Specify the end time for the position history in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. By default, the value will be the time that the request is made.</p>  <p>Requirement:</p>  <ul>   <li> <p>The time specified for <code>EndTimeExclusive</code> must be after the time for <code>StartTimeInclusive</code>.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetDevicePositionHistory::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetDevicePositionHistory::set_max_results): <p>An optional limit for the number of device positions returned in a single call.</p>  <p>Default value: <code>100</code> </p>
                            /// - On success, responds with [`GetDevicePositionHistoryOutput`](crate::output::GetDevicePositionHistoryOutput) with field(s):
    ///   - [`device_positions(Option<Vec<DevicePosition>>)`](crate::output::GetDevicePositionHistoryOutput::device_positions): <p>Contains the position history details for the requested device.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetDevicePositionHistoryOutput::next_token): <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results.</p>
                            /// - On failure, responds with [`SdkError<GetDevicePositionHistoryError>`](crate::error::GetDevicePositionHistoryError)
    pub fn get_device_position_history(&self) -> crate::client::fluent_builders::GetDevicePositionHistory {
                                crate::client::fluent_builders::GetDevicePositionHistory::new(self.handle.clone())
                            }
}

