// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableUser`](crate::client::fluent_builders::EnableUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::EnableUser::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::EnableUser::set_user_id): <p>The unique identifier for the user account that you want to enable.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::EnableUser::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::EnableUser::set_client_token): <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
                            /// - On success, responds with [`EnableUserOutput`](crate::output::EnableUserOutput) with field(s):
    ///   - [`user_id(Option<String>)`](crate::output::EnableUserOutput::user_id): <p>The unique identifier for the enabled user account.</p>
                            /// - On failure, responds with [`SdkError<EnableUserError>`](crate::error::EnableUserError)
    pub fn enable_user(&self) -> crate::client::fluent_builders::EnableUser {
                                crate::client::fluent_builders::EnableUser::new(self.handle.clone())
                            }
}

