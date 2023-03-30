// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateControlPanel`](crate::client::fluent_builders::UpdateControlPanel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`control_panel_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateControlPanel::control_panel_arn) / [`set_control_panel_arn(Option<String>)`](crate::client::fluent_builders::UpdateControlPanel::set_control_panel_arn): <p>The Amazon Resource Name (ARN) of the control panel.</p>
    ///   - [`control_panel_name(impl Into<String>)`](crate::client::fluent_builders::UpdateControlPanel::control_panel_name) / [`set_control_panel_name(Option<String>)`](crate::client::fluent_builders::UpdateControlPanel::set_control_panel_name): <p>The name of the control panel.</p>
                            /// - On success, responds with [`UpdateControlPanelOutput`](crate::output::UpdateControlPanelOutput) with field(s):
    ///   - [`control_panel(Option<ControlPanel>)`](crate::output::UpdateControlPanelOutput::control_panel): <p>The control panel to update.</p>
                            /// - On failure, responds with [`SdkError<UpdateControlPanelError>`](crate::error::UpdateControlPanelError)
    pub fn update_control_panel(&self) -> crate::client::fluent_builders::UpdateControlPanel {
                                crate::client::fluent_builders::UpdateControlPanel::new(self.handle.clone())
                            }
}

