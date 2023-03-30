// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateUser`](crate::client::fluent_builders::AssociateUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::AssociateUser::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::AssociateUser::set_username): <p>The user name from the identity provider for the user.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::AssociateUser::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::AssociateUser::set_instance_id): <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    ///   - [`identity_provider(IdentityProvider)`](crate::client::fluent_builders::AssociateUser::identity_provider) / [`set_identity_provider(Option<IdentityProvider>)`](crate::client::fluent_builders::AssociateUser::set_identity_provider): <p>The identity provider of the user.</p>
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::AssociateUser::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::AssociateUser::set_domain): <p>The domain name of the user.</p>
                            /// - On success, responds with [`AssociateUserOutput`](crate::output::AssociateUserOutput) with field(s):
    ///   - [`instance_user_summary(Option<InstanceUserSummary>)`](crate::output::AssociateUserOutput::instance_user_summary): <p>Metadata that describes the associate user operation.</p>
                            /// - On failure, responds with [`SdkError<AssociateUserError>`](crate::error::AssociateUserError)
    pub fn associate_user(&self) -> crate::client::fluent_builders::AssociateUser {
                                crate::client::fluent_builders::AssociateUser::new(self.handle.clone())
                            }
}

