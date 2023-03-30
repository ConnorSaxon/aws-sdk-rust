// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDeviceJob`](crate::client::fluent_builders::DescribeDeviceJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::DescribeDeviceJob::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::DescribeDeviceJob::set_job_id): <p>The job's ID.</p>
                            /// - On success, responds with [`DescribeDeviceJobOutput`](crate::output::DescribeDeviceJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::output::DescribeDeviceJobOutput::job_id): <p>The job's ID.</p>
    ///   - [`device_id(Option<String>)`](crate::output::DescribeDeviceJobOutput::device_id): <p>The device's ID.</p>
    ///   - [`device_arn(Option<String>)`](crate::output::DescribeDeviceJobOutput::device_arn): <p>The device's ARN.</p>
    ///   - [`device_name(Option<String>)`](crate::output::DescribeDeviceJobOutput::device_name): <p>The device's name.</p>
    ///   - [`device_type(Option<DeviceType>)`](crate::output::DescribeDeviceJobOutput::device_type): <p>The device's type.</p>
    ///   - [`image_version(Option<String>)`](crate::output::DescribeDeviceJobOutput::image_version): <p>For an OTA job, the target version of the device software.</p>
    ///   - [`status(Option<UpdateProgress>)`](crate::output::DescribeDeviceJobOutput::status): <p>The job's status.</p>
    ///   - [`created_time(Option<DateTime>)`](crate::output::DescribeDeviceJobOutput::created_time): <p>When the job was created.</p>
    ///   - [`job_type(Option<JobType>)`](crate::output::DescribeDeviceJobOutput::job_type): <p>The job's type.</p>
                            /// - On failure, responds with [`SdkError<DescribeDeviceJobError>`](crate::error::DescribeDeviceJobError)
    pub fn describe_device_job(&self) -> crate::client::fluent_builders::DescribeDeviceJob {
                                crate::client::fluent_builders::DescribeDeviceJob::new(self.handle.clone())
                            }
}

