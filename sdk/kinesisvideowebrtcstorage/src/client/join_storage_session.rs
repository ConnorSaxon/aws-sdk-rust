// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`JoinStorageSession`](crate::client::fluent_builders::JoinStorageSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::JoinStorageSession::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::JoinStorageSession::set_channel_arn): <p> The Amazon Resource Name (ARN) of the signaling channel. </p>
                            /// - On success, responds with [`JoinStorageSessionOutput`](crate::output::JoinStorageSessionOutput)
                            /// - On failure, responds with [`SdkError<JoinStorageSessionError>`](crate::error::JoinStorageSessionError)
    pub fn join_storage_session(&self) -> crate::client::fluent_builders::JoinStorageSession {
                                crate::client::fluent_builders::JoinStorageSession::new(self.handle.clone())
                            }
}

