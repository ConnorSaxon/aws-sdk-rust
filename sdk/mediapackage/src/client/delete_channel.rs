// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteChannel`](crate::client::fluent_builders::DeleteChannel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteChannel::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteChannel::set_id): The ID of the Channel to delete.
                            /// - On success, responds with [`DeleteChannelOutput`](crate::output::DeleteChannelOutput)
                            /// - On failure, responds with [`SdkError<DeleteChannelError>`](crate::error::DeleteChannelError)
    pub fn delete_channel(&self) -> crate::client::fluent_builders::DeleteChannel {
                                crate::client::fluent_builders::DeleteChannel::new(self.handle.clone())
                            }
}

