// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::client::fluent_builders::DeleteUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_access_token): <p>A valid access token that Amazon Cognito issued to the user whose user profile you want to delete.</p>
                            /// - On success, responds with [`DeleteUserOutput`](crate::output::DeleteUserOutput)
                            /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::error::DeleteUserError)
    pub fn delete_user(&self) -> crate::client::fluent_builders::DeleteUser {
                                crate::client::fluent_builders::DeleteUser::new(self.handle.clone())
                            }
}

