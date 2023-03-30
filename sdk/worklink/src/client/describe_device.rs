// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDevice`](crate::client::fluent_builders::DescribeDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeDevice::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::client::fluent_builders::DescribeDevice::set_fleet_arn): <p>The ARN of the fleet.</p>
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::DescribeDevice::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::DescribeDevice::set_device_id): <p>A unique identifier for a registered user's device.</p>
                            /// - On success, responds with [`DescribeDeviceOutput`](crate::output::DescribeDeviceOutput) with field(s):
    ///   - [`status(Option<DeviceStatus>)`](crate::output::DescribeDeviceOutput::status): <p>The current state of the device.</p>
    ///   - [`model(Option<String>)`](crate::output::DescribeDeviceOutput::model): <p>The model of the device.</p>
    ///   - [`manufacturer(Option<String>)`](crate::output::DescribeDeviceOutput::manufacturer): <p>The manufacturer of the device.</p>
    ///   - [`operating_system(Option<String>)`](crate::output::DescribeDeviceOutput::operating_system): <p>The operating system of the device.</p>
    ///   - [`operating_system_version(Option<String>)`](crate::output::DescribeDeviceOutput::operating_system_version): <p>The operating system version of the device.</p>
    ///   - [`patch_level(Option<String>)`](crate::output::DescribeDeviceOutput::patch_level): <p>The operating system patch level of the device.</p>
    ///   - [`first_accessed_time(Option<DateTime>)`](crate::output::DescribeDeviceOutput::first_accessed_time): <p>The date that the device first signed in to Amazon WorkLink.</p>
    ///   - [`last_accessed_time(Option<DateTime>)`](crate::output::DescribeDeviceOutput::last_accessed_time): <p>The date that the device last accessed Amazon WorkLink.</p>
    ///   - [`username(Option<String>)`](crate::output::DescribeDeviceOutput::username): <p>The user name associated with the device.</p>
                            /// - On failure, responds with [`SdkError<DescribeDeviceError>`](crate::error::DescribeDeviceError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn describe_device(&self) -> crate::client::fluent_builders::DescribeDevice {
                                crate::client::fluent_builders::DescribeDevice::new(self.handle.clone())
                            }
}

