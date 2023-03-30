// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAction`](crate::client::fluent_builders::GetAction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetAction::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetAction::set_id): <p>The ID of the action.</p>
                            /// - On success, responds with [`GetActionOutput`](crate::output::GetActionOutput) with field(s):
    ///   - [`action(Option<Action>)`](crate::output::GetActionOutput::action): <p>Information about the action.</p>
                            /// - On failure, responds with [`SdkError<GetActionError>`](crate::error::GetActionError)
    pub fn get_action(&self) -> crate::client::fluent_builders::GetAction {
                                crate::client::fluent_builders::GetAction::new(self.handle.clone())
                            }
}

