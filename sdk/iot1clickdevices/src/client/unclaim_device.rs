// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UnclaimDevice`](crate::client::fluent_builders::UnclaimDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::UnclaimDevice::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::UnclaimDevice::set_device_id): <p>The unique identifier of the device.</p>
                            /// - On success, responds with [`UnclaimDeviceOutput`](crate::output::UnclaimDeviceOutput) with field(s):
    ///   - [`state(Option<String>)`](crate::output::UnclaimDeviceOutput::state): <p>The device's final claim state.</p>
                            /// - On failure, responds with [`SdkError<UnclaimDeviceError>`](crate::error::UnclaimDeviceError)
    pub fn unclaim_device(&self) -> crate::client::fluent_builders::UnclaimDevice {
                                crate::client::fluent_builders::UnclaimDevice::new(self.handle.clone())
                            }
}

