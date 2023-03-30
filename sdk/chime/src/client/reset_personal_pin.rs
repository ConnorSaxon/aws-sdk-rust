// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ResetPersonalPIN`](crate::client::fluent_builders::ResetPersonalPIN) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ResetPersonalPIN::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ResetPersonalPIN::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::ResetPersonalPIN::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::ResetPersonalPIN::set_user_id): <p>The user ID.</p>
                            /// - On success, responds with [`ResetPersonalPinOutput`](crate::output::ResetPersonalPinOutput) with field(s):
    ///   - [`user(Option<User>)`](crate::output::ResetPersonalPinOutput::user): <p>The user details and new personal meeting PIN.</p>
                            /// - On failure, responds with [`SdkError<ResetPersonalPINError>`](crate::error::ResetPersonalPINError)
    pub fn reset_personal_pin(&self) -> crate::client::fluent_builders::ResetPersonalPIN {
                                crate::client::fluent_builders::ResetPersonalPIN::new(self.handle.clone())
                            }
}

