// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AdminSetUserMFAPreference`](crate::client::fluent_builders::AdminSetUserMFAPreference) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`sms_mfa_settings(SmsMfaSettingsType)`](crate::client::fluent_builders::AdminSetUserMFAPreference::sms_mfa_settings) / [`set_sms_mfa_settings(Option<SmsMfaSettingsType>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::set_sms_mfa_settings): <p>The SMS text message MFA settings.</p>
    ///   - [`software_token_mfa_settings(SoftwareTokenMfaSettingsType)`](crate::client::fluent_builders::AdminSetUserMFAPreference::software_token_mfa_settings) / [`set_software_token_mfa_settings(Option<SoftwareTokenMfaSettingsType>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::set_software_token_mfa_settings): <p>The time-based one-time password software token MFA settings.</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::set_username): <p>The user pool username or alias.</p>
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::AdminSetUserMFAPreference::set_user_pool_id): <p>The user pool ID.</p>
                            /// - On success, responds with [`AdminSetUserMfaPreferenceOutput`](crate::output::AdminSetUserMfaPreferenceOutput)
                            /// - On failure, responds with [`SdkError<AdminSetUserMFAPreferenceError>`](crate::error::AdminSetUserMFAPreferenceError)
    pub fn admin_set_user_mfa_preference(&self) -> crate::client::fluent_builders::AdminSetUserMFAPreference {
                                crate::client::fluent_builders::AdminSetUserMFAPreference::new(self.handle.clone())
                            }
}

