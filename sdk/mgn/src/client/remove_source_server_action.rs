// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveSourceServerAction`](crate::client::fluent_builders::RemoveSourceServerAction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_server_id(impl Into<String>)`](crate::client::fluent_builders::RemoveSourceServerAction::source_server_id) / [`set_source_server_id(Option<String>)`](crate::client::fluent_builders::RemoveSourceServerAction::set_source_server_id): <p>Source server ID of the post migration custom action to remove.</p>
    ///   - [`action_id(impl Into<String>)`](crate::client::fluent_builders::RemoveSourceServerAction::action_id) / [`set_action_id(Option<String>)`](crate::client::fluent_builders::RemoveSourceServerAction::set_action_id): <p>Source server post migration custom action ID to remove.</p>
                            /// - On success, responds with [`RemoveSourceServerActionOutput`](crate::output::RemoveSourceServerActionOutput)
                            /// - On failure, responds with [`SdkError<RemoveSourceServerActionError>`](crate::error::RemoveSourceServerActionError)
    pub fn remove_source_server_action(&self) -> crate::client::fluent_builders::RemoveSourceServerAction {
                                crate::client::fluent_builders::RemoveSourceServerAction::new(self.handle.clone())
                            }
}

