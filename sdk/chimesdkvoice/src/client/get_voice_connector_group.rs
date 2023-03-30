// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVoiceConnectorGroup`](crate::client::fluent_builders::GetVoiceConnectorGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_group_id(impl Into<String>)`](crate::client::fluent_builders::GetVoiceConnectorGroup::voice_connector_group_id) / [`set_voice_connector_group_id(Option<String>)`](crate::client::fluent_builders::GetVoiceConnectorGroup::set_voice_connector_group_id): (undocumented)
                            /// - On success, responds with [`GetVoiceConnectorGroupOutput`](crate::output::GetVoiceConnectorGroupOutput) with field(s):
    ///   - [`voice_connector_group(Option<VoiceConnectorGroup>)`](crate::output::GetVoiceConnectorGroupOutput::voice_connector_group): (undocumented)
                            /// - On failure, responds with [`SdkError<GetVoiceConnectorGroupError>`](crate::error::GetVoiceConnectorGroupError)
    pub fn get_voice_connector_group(&self) -> crate::client::fluent_builders::GetVoiceConnectorGroup {
                                crate::client::fluent_builders::GetVoiceConnectorGroup::new(self.handle.clone())
                            }
}

