// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ResetPassword`](crate::client::fluent_builders::ResetPassword) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::ResetPassword::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::ResetPassword::set_organization_id): <p>The identifier of the organization that contains the user for which the password is reset.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::ResetPassword::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::ResetPassword::set_user_id): <p>The identifier of the user for whom the password is reset.</p>
    ///   - [`password(impl Into<String>)`](crate::client::fluent_builders::ResetPassword::password) / [`set_password(Option<String>)`](crate::client::fluent_builders::ResetPassword::set_password): <p>The new password for the user.</p>
                            /// - On success, responds with [`ResetPasswordOutput`](crate::output::ResetPasswordOutput)
                            /// - On failure, responds with [`SdkError<ResetPasswordError>`](crate::error::ResetPasswordError)
    pub fn reset_password(&self) -> crate::client::fluent_builders::ResetPassword {
                                crate::client::fluent_builders::ResetPassword::new(self.handle.clone())
                            }
}

