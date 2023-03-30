// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SignOutUser`](crate::client::fluent_builders::SignOutUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::client::fluent_builders::SignOutUser::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::client::fluent_builders::SignOutUser::set_fleet_arn): <p>The ARN of the fleet.</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::SignOutUser::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::SignOutUser::set_username): <p>The name of the user.</p>
                            /// - On success, responds with [`SignOutUserOutput`](crate::output::SignOutUserOutput)
                            /// - On failure, responds with [`SdkError<SignOutUserError>`](crate::error::SignOutUserError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn sign_out_user(&self) -> crate::client::fluent_builders::SignOutUser {
                                crate::client::fluent_builders::SignOutUser::new(self.handle.clone())
                            }
}

