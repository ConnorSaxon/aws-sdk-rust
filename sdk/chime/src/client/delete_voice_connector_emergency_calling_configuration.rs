// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteVoiceConnectorEmergencyCallingConfiguration`](crate::client::fluent_builders::DeleteVoiceConnectorEmergencyCallingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::DeleteVoiceConnectorEmergencyCallingConfiguration::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::DeleteVoiceConnectorEmergencyCallingConfiguration::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
                            /// - On success, responds with [`DeleteVoiceConnectorEmergencyCallingConfigurationOutput`](crate::output::DeleteVoiceConnectorEmergencyCallingConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteVoiceConnectorEmergencyCallingConfigurationError>`](crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError)
    pub fn delete_voice_connector_emergency_calling_configuration(&self) -> crate::client::fluent_builders::DeleteVoiceConnectorEmergencyCallingConfiguration {
                                crate::client::fluent_builders::DeleteVoiceConnectorEmergencyCallingConfiguration::new(self.handle.clone())
                            }
}

