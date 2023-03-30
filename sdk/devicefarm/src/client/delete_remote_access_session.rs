// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRemoteAccessSession`](crate::client::fluent_builders::DeleteRemoteAccessSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DeleteRemoteAccessSession::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DeleteRemoteAccessSession::set_arn): <p>The Amazon Resource Name (ARN) of the session for which you want to delete remote access.</p>
                            /// - On success, responds with [`DeleteRemoteAccessSessionOutput`](crate::output::DeleteRemoteAccessSessionOutput)
                            /// - On failure, responds with [`SdkError<DeleteRemoteAccessSessionError>`](crate::error::DeleteRemoteAccessSessionError)
    pub fn delete_remote_access_session(&self) -> crate::client::fluent_builders::DeleteRemoteAccessSession {
                                crate::client::fluent_builders::DeleteRemoteAccessSession::new(self.handle.clone())
                            }
}

