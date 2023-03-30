// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDevicesJobs`](crate::client::fluent_builders::ListDevicesJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDevicesJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::ListDevicesJobs::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::ListDevicesJobs::set_device_id): <p>Filter results by the job's target device ID.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDevicesJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDevicesJobs::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDevicesJobs::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListDevicesJobs::set_max_results): <p>The maximum number of device jobs to return in one page of results.</p>
                            /// - On success, responds with [`ListDevicesJobsOutput`](crate::output::ListDevicesJobsOutput) with field(s):
    ///   - [`device_jobs(Option<Vec<DeviceJob>>)`](crate::output::ListDevicesJobsOutput::device_jobs): <p>A list of jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDevicesJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
                            /// - On failure, responds with [`SdkError<ListDevicesJobsError>`](crate::error::ListDevicesJobsError)
    pub fn list_devices_jobs(&self) -> crate::client::fluent_builders::ListDevicesJobs {
                                crate::client::fluent_builders::ListDevicesJobs::new(self.handle.clone())
                            }
}

