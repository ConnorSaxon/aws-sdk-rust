// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelInputDeviceTransfer`](crate::client::fluent_builders::CancelInputDeviceTransfer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`input_device_id(impl Into<String>)`](crate::client::fluent_builders::CancelInputDeviceTransfer::input_device_id) / [`set_input_device_id(Option<String>)`](crate::client::fluent_builders::CancelInputDeviceTransfer::set_input_device_id): The unique ID of the input device to cancel. For example, hd-123456789abcdef.
                            /// - On success, responds with [`CancelInputDeviceTransferOutput`](crate::output::CancelInputDeviceTransferOutput)
                            /// - On failure, responds with [`SdkError<CancelInputDeviceTransferError>`](crate::error::CancelInputDeviceTransferError)
    pub fn cancel_input_device_transfer(&self) -> crate::client::fluent_builders::CancelInputDeviceTransfer {
                                crate::client::fluent_builders::CancelInputDeviceTransfer::new(self.handle.clone())
                            }
}

