// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ValidateConfigurationSettings`](crate::client::fluent_builders::ValidateConfigurationSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::set_application_name): <p>The name of the application that the configuration template or environment belongs to.</p>
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::set_template_name): <p>The name of the configuration template to validate the settings against.</p>  <p>Condition: You cannot specify both this and an environment name.</p>
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::ValidateConfigurationSettings::set_environment_name): <p>The name of the environment to validate the settings against.</p>  <p>Condition: You cannot specify both this and a configuration template name.</p>
    ///   - [`option_settings(Vec<ConfigurationOptionSetting>)`](crate::client::fluent_builders::ValidateConfigurationSettings::option_settings) / [`set_option_settings(Option<Vec<ConfigurationOptionSetting>>)`](crate::client::fluent_builders::ValidateConfigurationSettings::set_option_settings): <p>A list of the options and desired values to evaluate.</p>
                            /// - On success, responds with [`ValidateConfigurationSettingsOutput`](crate::output::ValidateConfigurationSettingsOutput) with field(s):
    ///   - [`messages(Option<Vec<ValidationMessage>>)`](crate::output::ValidateConfigurationSettingsOutput::messages): <p> A list of <code>ValidationMessage</code>. </p>
                            /// - On failure, responds with [`SdkError<ValidateConfigurationSettingsError>`](crate::error::ValidateConfigurationSettingsError)
    pub fn validate_configuration_settings(&self) -> crate::client::fluent_builders::ValidateConfigurationSettings {
                                crate::client::fluent_builders::ValidateConfigurationSettings::new(self.handle.clone())
                            }
}

