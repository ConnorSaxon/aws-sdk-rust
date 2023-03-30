// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutVoiceConnectorTermination`](crate::client::fluent_builders::PutVoiceConnectorTermination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::PutVoiceConnectorTermination::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::PutVoiceConnectorTermination::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    ///   - [`termination(Termination)`](crate::client::fluent_builders::PutVoiceConnectorTermination::termination) / [`set_termination(Option<Termination>)`](crate::client::fluent_builders::PutVoiceConnectorTermination::set_termination): <p>The termination setting details to add.</p>
                            /// - On success, responds with [`PutVoiceConnectorTerminationOutput`](crate::output::PutVoiceConnectorTerminationOutput) with field(s):
    ///   - [`termination(Option<Termination>)`](crate::output::PutVoiceConnectorTerminationOutput::termination): <p>The updated termination setting details.</p>
                            /// - On failure, responds with [`SdkError<PutVoiceConnectorTerminationError>`](crate::error::PutVoiceConnectorTerminationError)
    pub fn put_voice_connector_termination(&self) -> crate::client::fluent_builders::PutVoiceConnectorTermination {
                                crate::client::fluent_builders::PutVoiceConnectorTermination::new(self.handle.clone())
                            }
}

