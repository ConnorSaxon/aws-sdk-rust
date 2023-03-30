// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartInputDeviceMaintenanceWindow`](crate::client::fluent_builders::StartInputDeviceMaintenanceWindow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`input_device_id(impl Into<String>)`](crate::client::fluent_builders::StartInputDeviceMaintenanceWindow::input_device_id) / [`set_input_device_id(Option<String>)`](crate::client::fluent_builders::StartInputDeviceMaintenanceWindow::set_input_device_id): The unique ID of the input device to start a maintenance window for. For example, hd-123456789abcdef.
                            /// - On success, responds with [`StartInputDeviceMaintenanceWindowOutput`](crate::output::StartInputDeviceMaintenanceWindowOutput)
                            /// - On failure, responds with [`SdkError<StartInputDeviceMaintenanceWindowError>`](crate::error::StartInputDeviceMaintenanceWindowError)
    pub fn start_input_device_maintenance_window(&self) -> crate::client::fluent_builders::StartInputDeviceMaintenanceWindow {
                                crate::client::fluent_builders::StartInputDeviceMaintenanceWindow::new(self.handle.clone())
                            }
}

