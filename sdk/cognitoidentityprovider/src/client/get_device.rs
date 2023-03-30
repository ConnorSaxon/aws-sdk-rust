// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDevice`](crate::client::fluent_builders::GetDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_key(impl Into<String>)`](crate::client::fluent_builders::GetDevice::device_key) / [`set_device_key(Option<String>)`](crate::client::fluent_builders::GetDevice::set_device_key): <p>The device key.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::GetDevice::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::GetDevice::set_access_token): <p>A valid access token that Amazon Cognito issued to the user whose device information you want to request.</p>
                            /// - On success, responds with [`GetDeviceOutput`](crate::output::GetDeviceOutput) with field(s):
    ///   - [`device(Option<DeviceType>)`](crate::output::GetDeviceOutput::device): <p>The device.</p>
                            /// - On failure, responds with [`SdkError<GetDeviceError>`](crate::error::GetDeviceError)
    pub fn get_device(&self) -> crate::client::fluent_builders::GetDevice {
                                crate::client::fluent_builders::GetDevice::new(self.handle.clone())
                            }
}

