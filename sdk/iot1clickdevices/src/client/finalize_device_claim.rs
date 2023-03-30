// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`FinalizeDeviceClaim`](crate::client::fluent_builders::FinalizeDeviceClaim) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::FinalizeDeviceClaim::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::FinalizeDeviceClaim::set_device_id): <p>The unique identifier of the device.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::FinalizeDeviceClaim::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::FinalizeDeviceClaim::set_tags): <p>A collection of key/value pairs defining the resource tags. For example, { "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <p> </p>
                            /// - On success, responds with [`FinalizeDeviceClaimOutput`](crate::output::FinalizeDeviceClaimOutput) with field(s):
    ///   - [`state(Option<String>)`](crate::output::FinalizeDeviceClaimOutput::state): <p>The device's final claim state.</p>
                            /// - On failure, responds with [`SdkError<FinalizeDeviceClaimError>`](crate::error::FinalizeDeviceClaimError)
    pub fn finalize_device_claim(&self) -> crate::client::fluent_builders::FinalizeDeviceClaim {
                                crate::client::fluent_builders::FinalizeDeviceClaim::new(self.handle.clone())
                            }
}

