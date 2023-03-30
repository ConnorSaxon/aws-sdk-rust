// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateUserSettings`](crate::client::fluent_builders::AssociateUserSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateUserSettings::portal_arn) / [`set_portal_arn(Option<String>)`](crate::client::fluent_builders::AssociateUserSettings::set_portal_arn): <p>The ARN of the web portal.</p>
    ///   - [`user_settings_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateUserSettings::user_settings_arn) / [`set_user_settings_arn(Option<String>)`](crate::client::fluent_builders::AssociateUserSettings::set_user_settings_arn): <p>The ARN of the user settings.</p>
                            /// - On success, responds with [`AssociateUserSettingsOutput`](crate::output::AssociateUserSettingsOutput) with field(s):
    ///   - [`portal_arn(Option<String>)`](crate::output::AssociateUserSettingsOutput::portal_arn): <p>The ARN of the web portal.</p>
    ///   - [`user_settings_arn(Option<String>)`](crate::output::AssociateUserSettingsOutput::user_settings_arn): <p>The ARN of the user settings.</p>
                            /// - On failure, responds with [`SdkError<AssociateUserSettingsError>`](crate::error::AssociateUserSettingsError)
    pub fn associate_user_settings(&self) -> crate::client::fluent_builders::AssociateUserSettings {
                                crate::client::fluent_builders::AssociateUserSettings::new(self.handle.clone())
                            }
}

