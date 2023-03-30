// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AdminDisableProviderForUser`](crate::client::fluent_builders::AdminDisableProviderForUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::AdminDisableProviderForUser::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::AdminDisableProviderForUser::set_user_pool_id): <p>The user pool ID for the user pool.</p>
    ///   - [`user(ProviderUserIdentifierType)`](crate::client::fluent_builders::AdminDisableProviderForUser::user) / [`set_user(Option<ProviderUserIdentifierType>)`](crate::client::fluent_builders::AdminDisableProviderForUser::set_user): <p>The user to be disabled.</p>
                            /// - On success, responds with [`AdminDisableProviderForUserOutput`](crate::output::AdminDisableProviderForUserOutput)
                            /// - On failure, responds with [`SdkError<AdminDisableProviderForUserError>`](crate::error::AdminDisableProviderForUserError)
    pub fn admin_disable_provider_for_user(&self) -> crate::client::fluent_builders::AdminDisableProviderForUser {
                                crate::client::fluent_builders::AdminDisableProviderForUser::new(self.handle.clone())
                            }
}

