// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDevices`](crate::client::fluent_builders::ListDevices) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDevices::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::ListDevices::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::ListDevices::set_job_id): <p>The ID of the job used to order the device.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDevices::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDevices::set_max_results): <p>The maximum number of devices to list per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDevices::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDevices::set_next_token): <p>A pagination token to continue to the next page of results.</p>
                            /// - On success, responds with [`ListDevicesOutput`](crate::output::ListDevicesOutput) with field(s):
    ///   - [`devices(Option<Vec<DeviceSummary>>)`](crate::output::ListDevicesOutput::devices): <p>A list of device structures that contain information about the device.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDevicesOutput::next_token): <p>A pagination token to continue to the next page of devices.</p>
                            /// - On failure, responds with [`SdkError<ListDevicesError>`](crate::error::ListDevicesError)
    pub fn list_devices(&self) -> crate::client::fluent_builders::ListDevices {
                                crate::client::fluent_builders::ListDevices::new(self.handle.clone())
                            }
}

