// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDevice`](crate::client::fluent_builders::GetDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_arn(impl Into<String>)`](crate::client::fluent_builders::GetDevice::device_arn) / [`set_device_arn(Option<String>)`](crate::client::fluent_builders::GetDevice::set_device_arn): <p>The ARN of the device to retrieve.</p>
                            /// - On success, responds with [`GetDeviceOutput`](crate::output::GetDeviceOutput) with field(s):
    ///   - [`device_arn(Option<String>)`](crate::output::GetDeviceOutput::device_arn): <p>The ARN of the device.</p>
    ///   - [`device_name(Option<String>)`](crate::output::GetDeviceOutput::device_name): <p>The name of the device.</p>
    ///   - [`provider_name(Option<String>)`](crate::output::GetDeviceOutput::provider_name): <p>The name of the partner company for the device.</p>
    ///   - [`device_type(Option<DeviceType>)`](crate::output::GetDeviceOutput::device_type): <p>The type of the device.</p>
    ///   - [`device_status(Option<DeviceStatus>)`](crate::output::GetDeviceOutput::device_status): <p>The status of the device.</p>
    ///   - [`device_capabilities(Option<String>)`](crate::output::GetDeviceOutput::device_capabilities): <p>Details about the capabilities of the device.</p>
                            /// - On failure, responds with [`SdkError<GetDeviceError>`](crate::error::GetDeviceError)
    pub fn get_device(&self) -> crate::client::fluent_builders::GetDevice {
                                crate::client::fluent_builders::GetDevice::new(self.handle.clone())
                            }
}

