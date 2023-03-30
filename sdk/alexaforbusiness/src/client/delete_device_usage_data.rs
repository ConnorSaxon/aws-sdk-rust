// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDeviceUsageData`](crate::client::fluent_builders::DeleteDeviceUsageData) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteDeviceUsageData::device_arn) / [`set_device_arn(Option<String>)`](crate::client::fluent_builders::DeleteDeviceUsageData::set_device_arn): <p>The ARN of the device.</p>
    ///   - [`device_usage_type(DeviceUsageType)`](crate::client::fluent_builders::DeleteDeviceUsageData::device_usage_type) / [`set_device_usage_type(Option<DeviceUsageType>)`](crate::client::fluent_builders::DeleteDeviceUsageData::set_device_usage_type): <p>The type of usage data to delete.</p>
                            /// - On success, responds with [`DeleteDeviceUsageDataOutput`](crate::output::DeleteDeviceUsageDataOutput)
                            /// - On failure, responds with [`SdkError<DeleteDeviceUsageDataError>`](crate::error::DeleteDeviceUsageDataError)
    pub fn delete_device_usage_data(&self) -> crate::client::fluent_builders::DeleteDeviceUsageData {
                                crate::client::fluent_builders::DeleteDeviceUsageData::new(self.handle.clone())
                            }
}

