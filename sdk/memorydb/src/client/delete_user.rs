// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::client::fluent_builders::DeleteUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::DeleteUser::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::DeleteUser::set_user_name): <p>The name of the user to delete</p>
                            /// - On success, responds with [`DeleteUserOutput`](crate::output::DeleteUserOutput) with field(s):
    ///   - [`user(Option<User>)`](crate::output::DeleteUserOutput::user): <p>The user object that has been deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::error::DeleteUserError)
    pub fn delete_user(&self) -> crate::client::fluent_builders::DeleteUser {
                                crate::client::fluent_builders::DeleteUser::new(self.handle.clone())
                            }
}

