// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutVoiceConnectorOrigination`](crate::client::fluent_builders::PutVoiceConnectorOrigination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::PutVoiceConnectorOrigination::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::PutVoiceConnectorOrigination::set_voice_connector_id): (undocumented)
    ///   - [`origination(Origination)`](crate::client::fluent_builders::PutVoiceConnectorOrigination::origination) / [`set_origination(Option<Origination>)`](crate::client::fluent_builders::PutVoiceConnectorOrigination::set_origination): (undocumented)
                            /// - On success, responds with [`PutVoiceConnectorOriginationOutput`](crate::output::PutVoiceConnectorOriginationOutput) with field(s):
    ///   - [`origination(Option<Origination>)`](crate::output::PutVoiceConnectorOriginationOutput::origination): (undocumented)
                            /// - On failure, responds with [`SdkError<PutVoiceConnectorOriginationError>`](crate::error::PutVoiceConnectorOriginationError)
    pub fn put_voice_connector_origination(&self) -> crate::client::fluent_builders::PutVoiceConnectorOrigination {
                                crate::client::fluent_builders::PutVoiceConnectorOrigination::new(self.handle.clone())
                            }
}

