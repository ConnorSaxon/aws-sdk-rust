// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutVoiceConnectorLoggingConfiguration`](crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    ///   - [`logging_configuration(LoggingConfiguration)`](crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration::logging_configuration) / [`set_logging_configuration(Option<LoggingConfiguration>)`](crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration::set_logging_configuration): <p>The logging configuration details to add.</p>
                            /// - On success, responds with [`PutVoiceConnectorLoggingConfigurationOutput`](crate::output::PutVoiceConnectorLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::output::PutVoiceConnectorLoggingConfigurationOutput::logging_configuration): <p>The updated logging configuration details.</p>
                            /// - On failure, responds with [`SdkError<PutVoiceConnectorLoggingConfigurationError>`](crate::error::PutVoiceConnectorLoggingConfigurationError)
    pub fn put_voice_connector_logging_configuration(&self) -> crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration {
                                crate::client::fluent_builders::PutVoiceConnectorLoggingConfiguration::new(self.handle.clone())
                            }
}

