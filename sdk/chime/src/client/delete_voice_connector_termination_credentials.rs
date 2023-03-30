// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteVoiceConnectorTerminationCredentials`](crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    ///   - [`usernames(Vec<String>)`](crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials::usernames) / [`set_usernames(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials::set_usernames): <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
                            /// - On success, responds with [`DeleteVoiceConnectorTerminationCredentialsOutput`](crate::output::DeleteVoiceConnectorTerminationCredentialsOutput)
                            /// - On failure, responds with [`SdkError<DeleteVoiceConnectorTerminationCredentialsError>`](crate::error::DeleteVoiceConnectorTerminationCredentialsError)
    pub fn delete_voice_connector_termination_credentials(&self) -> crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials {
                                crate::client::fluent_builders::DeleteVoiceConnectorTerminationCredentials::new(self.handle.clone())
                            }
}

