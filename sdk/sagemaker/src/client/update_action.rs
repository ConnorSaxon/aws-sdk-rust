// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAction`](crate::client::fluent_builders::UpdateAction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`action_name(impl Into<String>)`](crate::client::fluent_builders::UpdateAction::action_name) / [`set_action_name(Option<String>)`](crate::client::fluent_builders::UpdateAction::set_action_name): <p>The name of the action to update.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateAction::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateAction::set_description): <p>The new description for the action.</p>
    ///   - [`status(ActionStatus)`](crate::client::fluent_builders::UpdateAction::status) / [`set_status(Option<ActionStatus>)`](crate::client::fluent_builders::UpdateAction::set_status): <p>The new status for the action.</p>
    ///   - [`properties(HashMap<String, String>)`](crate::client::fluent_builders::UpdateAction::properties) / [`set_properties(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateAction::set_properties): <p>The new list of properties. Overwrites the current property list.</p>
    ///   - [`properties_to_remove(Vec<String>)`](crate::client::fluent_builders::UpdateAction::properties_to_remove) / [`set_properties_to_remove(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateAction::set_properties_to_remove): <p>A list of properties to remove.</p>
                            /// - On success, responds with [`UpdateActionOutput`](crate::output::UpdateActionOutput) with field(s):
    ///   - [`action_arn(Option<String>)`](crate::output::UpdateActionOutput::action_arn): <p>The Amazon Resource Name (ARN) of the action.</p>
                            /// - On failure, responds with [`SdkError<UpdateActionError>`](crate::error::UpdateActionError)
    pub fn update_action(&self) -> crate::client::fluent_builders::UpdateAction {
                                crate::client::fluent_builders::UpdateAction::new(self.handle.clone())
                            }
}

