// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAccountSettings`](crate::client::fluent_builders::GetAccountSettings) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetAccountSettings::send) it.
                            /// - On success, responds with [`GetAccountSettingsOutput`](crate::output::GetAccountSettingsOutput) with field(s):
    ///   - [`account_settings(Option<AccountSettings>)`](crate::output::GetAccountSettingsOutput::account_settings): <p>The Proton pipeline service role detail data that's returned by Proton.</p>
                            /// - On failure, responds with [`SdkError<GetAccountSettingsError>`](crate::error::GetAccountSettingsError)
    pub fn get_account_settings(&self) -> crate::client::fluent_builders::GetAccountSettings {
                                crate::client::fluent_builders::GetAccountSettings::new(self.handle.clone())
                            }
}

