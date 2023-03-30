// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDevice`](crate::client::fluent_builders::GetDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetDevice::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetDevice::set_arn): <p>The device type's ARN.</p>
                            /// - On success, responds with [`GetDeviceOutput`](crate::output::GetDeviceOutput) with field(s):
    ///   - [`device(Option<Device>)`](crate::output::GetDeviceOutput::device): <p>An object that contains information about the requested device.</p>
                            /// - On failure, responds with [`SdkError<GetDeviceError>`](crate::error::GetDeviceError)
    pub fn get_device(&self) -> crate::client::fluent_builders::GetDevice {
                                crate::client::fluent_builders::GetDevice::new(self.handle.clone())
                            }
}

