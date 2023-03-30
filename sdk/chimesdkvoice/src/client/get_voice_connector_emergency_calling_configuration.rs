// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVoiceConnectorEmergencyCallingConfiguration`](crate::client::fluent_builders::GetVoiceConnectorEmergencyCallingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::GetVoiceConnectorEmergencyCallingConfiguration::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::GetVoiceConnectorEmergencyCallingConfiguration::set_voice_connector_id): (undocumented)
                            /// - On success, responds with [`GetVoiceConnectorEmergencyCallingConfigurationOutput`](crate::output::GetVoiceConnectorEmergencyCallingConfigurationOutput) with field(s):
    ///   - [`emergency_calling_configuration(Option<EmergencyCallingConfiguration>)`](crate::output::GetVoiceConnectorEmergencyCallingConfigurationOutput::emergency_calling_configuration): (undocumented)
                            /// - On failure, responds with [`SdkError<GetVoiceConnectorEmergencyCallingConfigurationError>`](crate::error::GetVoiceConnectorEmergencyCallingConfigurationError)
    pub fn get_voice_connector_emergency_calling_configuration(&self) -> crate::client::fluent_builders::GetVoiceConnectorEmergencyCallingConfiguration {
                                crate::client::fluent_builders::GetVoiceConnectorEmergencyCallingConfiguration::new(self.handle.clone())
                            }
}

