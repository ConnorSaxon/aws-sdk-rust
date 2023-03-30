// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateUserAccessLoggingSettings`](crate::client::fluent_builders::AssociateUserAccessLoggingSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateUserAccessLoggingSettings::portal_arn) / [`set_portal_arn(Option<String>)`](crate::client::fluent_builders::AssociateUserAccessLoggingSettings::set_portal_arn): <p>The ARN of the web portal.</p>
    ///   - [`user_access_logging_settings_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateUserAccessLoggingSettings::user_access_logging_settings_arn) / [`set_user_access_logging_settings_arn(Option<String>)`](crate::client::fluent_builders::AssociateUserAccessLoggingSettings::set_user_access_logging_settings_arn): <p>The ARN of the user access logging settings.</p>
                            /// - On success, responds with [`AssociateUserAccessLoggingSettingsOutput`](crate::output::AssociateUserAccessLoggingSettingsOutput) with field(s):
    ///   - [`portal_arn(Option<String>)`](crate::output::AssociateUserAccessLoggingSettingsOutput::portal_arn): <p>The ARN of the web portal.</p>
    ///   - [`user_access_logging_settings_arn(Option<String>)`](crate::output::AssociateUserAccessLoggingSettingsOutput::user_access_logging_settings_arn): <p>The ARN of the user access logging settings.</p>
                            /// - On failure, responds with [`SdkError<AssociateUserAccessLoggingSettingsError>`](crate::error::AssociateUserAccessLoggingSettingsError)
    pub fn associate_user_access_logging_settings(&self) -> crate::client::fluent_builders::AssociateUserAccessLoggingSettings {
                                crate::client::fluent_builders::AssociateUserAccessLoggingSettings::new(self.handle.clone())
                            }
}

