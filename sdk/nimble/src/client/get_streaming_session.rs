// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetStreamingSession`](crate::client::fluent_builders::GetStreamingSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`session_id(impl Into<String>)`](crate::client::fluent_builders::GetStreamingSession::session_id) / [`set_session_id(Option<String>)`](crate::client::fluent_builders::GetStreamingSession::set_session_id): <p>The streaming session ID.</p>
    ///   - [`studio_id(impl Into<String>)`](crate::client::fluent_builders::GetStreamingSession::studio_id) / [`set_studio_id(Option<String>)`](crate::client::fluent_builders::GetStreamingSession::set_studio_id): <p>The studio ID. </p>
                            /// - On success, responds with [`GetStreamingSessionOutput`](crate::output::GetStreamingSessionOutput) with field(s):
    ///   - [`session(Option<StreamingSession>)`](crate::output::GetStreamingSessionOutput::session): <p>The session.</p>
                            /// - On failure, responds with [`SdkError<GetStreamingSessionError>`](crate::error::GetStreamingSessionError)
    pub fn get_streaming_session(&self) -> crate::client::fluent_builders::GetStreamingSession {
                                crate::client::fluent_builders::GetStreamingSession::new(self.handle.clone())
                            }
}

