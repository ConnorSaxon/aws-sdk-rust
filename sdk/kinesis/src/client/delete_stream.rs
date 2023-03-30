// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteStream`](crate::client::fluent_builders::DeleteStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::client::fluent_builders::DeleteStream::stream_name) / [`set_stream_name(Option<String>)`](crate::client::fluent_builders::DeleteStream::set_stream_name): <p>The name of the stream to delete.</p>
    ///   - [`enforce_consumer_deletion(bool)`](crate::client::fluent_builders::DeleteStream::enforce_consumer_deletion) / [`set_enforce_consumer_deletion(Option<bool>)`](crate::client::fluent_builders::DeleteStream::set_enforce_consumer_deletion): <p>If this parameter is unset (<code>null</code>) or if you set it to <code>false</code>, and the stream has registered consumers, the call to <code>DeleteStream</code> fails with a <code>ResourceInUseException</code>. </p>
    ///   - [`stream_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteStream::stream_arn) / [`set_stream_arn(Option<String>)`](crate::client::fluent_builders::DeleteStream::set_stream_arn): <p>The ARN of the stream.</p>
                            /// - On success, responds with [`DeleteStreamOutput`](crate::output::DeleteStreamOutput)
                            /// - On failure, responds with [`SdkError<DeleteStreamError>`](crate::error::DeleteStreamError)
    pub fn delete_stream(&self) -> crate::client::fluent_builders::DeleteStream {
                                crate::client::fluent_builders::DeleteStream::new(self.handle.clone())
                            }
}

