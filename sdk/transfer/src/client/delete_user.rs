// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::client::fluent_builders::DeleteUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`server_id(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::server_id) / [`set_server_id(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_server_id): <p>A system-assigned unique identifier for a server instance that has the user assigned to it.</p>
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_user_name): <p>A unique string that identifies a user that is being deleted from a server.</p>
                            /// - On success, responds with [`DeleteUserOutput`](crate::output::DeleteUserOutput)
                            /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::error::DeleteUserError)
    pub fn delete_user(&self) -> crate::client::fluent_builders::DeleteUser {
                                crate::client::fluent_builders::DeleteUser::new(self.handle.clone())
                            }
}

