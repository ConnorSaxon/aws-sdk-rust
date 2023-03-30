// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVoiceConnectorProxy`](crate::client::fluent_builders::GetVoiceConnectorProxy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::GetVoiceConnectorProxy::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::GetVoiceConnectorProxy::set_voice_connector_id): <p>The Amazon Chime voice connector ID.</p>
                            /// - On success, responds with [`GetVoiceConnectorProxyOutput`](crate::output::GetVoiceConnectorProxyOutput) with field(s):
    ///   - [`proxy(Option<Proxy>)`](crate::output::GetVoiceConnectorProxyOutput::proxy): <p>The proxy configuration details.</p>
                            /// - On failure, responds with [`SdkError<GetVoiceConnectorProxyError>`](crate::error::GetVoiceConnectorProxyError)
    pub fn get_voice_connector_proxy(&self) -> crate::client::fluent_builders::GetVoiceConnectorProxy {
                                crate::client::fluent_builders::GetVoiceConnectorProxy::new(self.handle.clone())
                            }
}

