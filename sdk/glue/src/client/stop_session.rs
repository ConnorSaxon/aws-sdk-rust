// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopSession`](crate::client::fluent_builders::StopSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::StopSession::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::StopSession::set_id): <p>The ID of the session to be stopped.</p>
    ///   - [`request_origin(impl Into<String>)`](crate::client::fluent_builders::StopSession::request_origin) / [`set_request_origin(Option<String>)`](crate::client::fluent_builders::StopSession::set_request_origin): <p>The origin of the request.</p>
                            /// - On success, responds with [`StopSessionOutput`](crate::output::StopSessionOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::StopSessionOutput::id): <p>Returns the Id of the stopped session.</p>
                            /// - On failure, responds with [`SdkError<StopSessionError>`](crate::error::StopSessionError)
    pub fn stop_session(&self) -> crate::client::fluent_builders::StopSession {
                                crate::client::fluent_builders::StopSession::new(self.handle.clone())
                            }
}

