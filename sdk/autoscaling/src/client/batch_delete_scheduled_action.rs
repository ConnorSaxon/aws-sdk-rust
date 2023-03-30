// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDeleteScheduledAction`](crate::client::fluent_builders::BatchDeleteScheduledAction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteScheduledAction::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::BatchDeleteScheduledAction::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`scheduled_action_names(Vec<String>)`](crate::client::fluent_builders::BatchDeleteScheduledAction::scheduled_action_names) / [`set_scheduled_action_names(Option<Vec<String>>)`](crate::client::fluent_builders::BatchDeleteScheduledAction::set_scheduled_action_names): <p>The names of the scheduled actions to delete. The maximum number allowed is 50. </p>
                            /// - On success, responds with [`BatchDeleteScheduledActionOutput`](crate::output::BatchDeleteScheduledActionOutput) with field(s):
    ///   - [`failed_scheduled_actions(Option<Vec<FailedScheduledUpdateGroupActionRequest>>)`](crate::output::BatchDeleteScheduledActionOutput::failed_scheduled_actions): <p>The names of the scheduled actions that could not be deleted, including an error message.</p>
                            /// - On failure, responds with [`SdkError<BatchDeleteScheduledActionError>`](crate::error::BatchDeleteScheduledActionError)
    pub fn batch_delete_scheduled_action(&self) -> crate::client::fluent_builders::BatchDeleteScheduledAction {
                                crate::client::fluent_builders::BatchDeleteScheduledAction::new(self.handle.clone())
                            }
}

