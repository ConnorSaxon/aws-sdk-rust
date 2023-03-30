// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTrial`](crate::client::fluent_builders::UpdateTrial) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`trial_name(impl Into<String>)`](crate::client::fluent_builders::UpdateTrial::trial_name) / [`set_trial_name(Option<String>)`](crate::client::fluent_builders::UpdateTrial::set_trial_name): <p>The name of the trial to update.</p>
    ///   - [`display_name(impl Into<String>)`](crate::client::fluent_builders::UpdateTrial::display_name) / [`set_display_name(Option<String>)`](crate::client::fluent_builders::UpdateTrial::set_display_name): <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
                            /// - On success, responds with [`UpdateTrialOutput`](crate::output::UpdateTrialOutput) with field(s):
    ///   - [`trial_arn(Option<String>)`](crate::output::UpdateTrialOutput::trial_arn): <p>The Amazon Resource Name (ARN) of the trial.</p>
                            /// - On failure, responds with [`SdkError<UpdateTrialError>`](crate::error::UpdateTrialError)
    pub fn update_trial(&self) -> crate::client::fluent_builders::UpdateTrial {
                                crate::client::fluent_builders::UpdateTrial::new(self.handle.clone())
                            }
}

