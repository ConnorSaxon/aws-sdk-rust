// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisconnectUser`](crate::client::fluent_builders::DisconnectUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`room_identifier(impl Into<String>)`](crate::client::fluent_builders::DisconnectUser::room_identifier) / [`set_room_identifier(Option<String>)`](crate::client::fluent_builders::DisconnectUser::set_room_identifier): <p>Identifier of the room from which the user's clients should be disconnected. Currently this must be an ARN.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::DisconnectUser::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::DisconnectUser::set_user_id): <p>ID of the user (connection) to disconnect from the room.</p>
    ///   - [`reason(impl Into<String>)`](crate::client::fluent_builders::DisconnectUser::reason) / [`set_reason(Option<String>)`](crate::client::fluent_builders::DisconnectUser::set_reason): <p>Reason for disconnecting the user.</p>
                            /// - On success, responds with [`DisconnectUserOutput`](crate::output::DisconnectUserOutput)
                            /// - On failure, responds with [`SdkError<DisconnectUserError>`](crate::error::DisconnectUserError)
    pub fn disconnect_user(&self) -> crate::client::fluent_builders::DisconnectUser {
                                crate::client::fluent_builders::DisconnectUser::new(self.handle.clone())
                            }
}

