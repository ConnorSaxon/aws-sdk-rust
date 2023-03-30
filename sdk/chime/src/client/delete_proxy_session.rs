// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteProxySession`](crate::client::fluent_builders::DeleteProxySession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::client::fluent_builders::DeleteProxySession::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::client::fluent_builders::DeleteProxySession::set_voice_connector_id): <p>The Amazon Chime voice connector ID.</p>
    ///   - [`proxy_session_id(impl Into<String>)`](crate::client::fluent_builders::DeleteProxySession::proxy_session_id) / [`set_proxy_session_id(Option<String>)`](crate::client::fluent_builders::DeleteProxySession::set_proxy_session_id): <p>The proxy session ID.</p>
                            /// - On success, responds with [`DeleteProxySessionOutput`](crate::output::DeleteProxySessionOutput)
                            /// - On failure, responds with [`SdkError<DeleteProxySessionError>`](crate::error::DeleteProxySessionError)
    pub fn delete_proxy_session(&self) -> crate::client::fluent_builders::DeleteProxySession {
                                crate::client::fluent_builders::DeleteProxySession::new(self.handle.clone())
                            }
}

