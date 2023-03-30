// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableControl`](crate::client::fluent_builders::DisableControl) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`control_identifier(impl Into<String>)`](crate::client::fluent_builders::DisableControl::control_identifier) / [`set_control_identifier(Option<String>)`](crate::client::fluent_builders::DisableControl::set_control_identifier): <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    ///   - [`target_identifier(impl Into<String>)`](crate::client::fluent_builders::DisableControl::target_identifier) / [`set_target_identifier(Option<String>)`](crate::client::fluent_builders::DisableControl::set_target_identifier): <p>The ARN of the organizational unit.</p>
                            /// - On success, responds with [`DisableControlOutput`](crate::output::DisableControlOutput) with field(s):
    ///   - [`operation_identifier(Option<String>)`](crate::output::DisableControlOutput::operation_identifier): <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
                            /// - On failure, responds with [`SdkError<DisableControlError>`](crate::error::DisableControlError)
    pub fn disable_control(&self) -> crate::client::fluent_builders::DisableControl {
                                crate::client::fluent_builders::DisableControl::new(self.handle.clone())
                            }
}

