// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRemoteAccessSession`](crate::client::fluent_builders::GetRemoteAccessSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetRemoteAccessSession::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetRemoteAccessSession::set_arn): <p>The Amazon Resource Name (ARN) of the remote access session about which you want to get session information.</p>
                            /// - On success, responds with [`GetRemoteAccessSessionOutput`](crate::output::GetRemoteAccessSessionOutput) with field(s):
    ///   - [`remote_access_session(Option<RemoteAccessSession>)`](crate::output::GetRemoteAccessSessionOutput::remote_access_session): <p>A container that lists detailed information about the remote access session.</p>
                            /// - On failure, responds with [`SdkError<GetRemoteAccessSessionError>`](crate::error::GetRemoteAccessSessionError)
    pub fn get_remote_access_session(&self) -> crate::client::fluent_builders::GetRemoteAccessSession {
                                crate::client::fluent_builders::GetRemoteAccessSession::new(self.handle.clone())
                            }
}

