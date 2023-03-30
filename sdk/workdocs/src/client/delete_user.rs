// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::client::fluent_builders::DeleteUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::authentication_token) / [`set_authentication_token(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_authentication_token): <p>Amazon WorkDocs authentication token. Do not set this field when using administrative API actions, as in accessing the API using AWS credentials.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_user_id): <p>The ID of the user.</p>
                            /// - On success, responds with [`DeleteUserOutput`](crate::output::DeleteUserOutput)
                            /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::error::DeleteUserError)
    pub fn delete_user(&self) -> crate::client::fluent_builders::DeleteUser {
                                crate::client::fluent_builders::DeleteUser::new(self.handle.clone())
                            }
}

