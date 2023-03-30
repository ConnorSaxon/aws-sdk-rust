// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDeviceProfile`](crate::client::fluent_builders::DeleteDeviceProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteDeviceProfile::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteDeviceProfile::set_id): <p>The ID of the resource to delete.</p>
                            /// - On success, responds with [`DeleteDeviceProfileOutput`](crate::output::DeleteDeviceProfileOutput)
                            /// - On failure, responds with [`SdkError<DeleteDeviceProfileError>`](crate::error::DeleteDeviceProfileError)
    pub fn delete_device_profile(&self) -> crate::client::fluent_builders::DeleteDeviceProfile {
                                crate::client::fluent_builders::DeleteDeviceProfile::new(self.handle.clone())
                            }
}

