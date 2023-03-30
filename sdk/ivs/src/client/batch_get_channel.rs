// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetChannel`](crate::client::fluent_builders::BatchGetChannel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arns(Vec<String>)`](crate::client::fluent_builders::BatchGetChannel::arns) / [`set_arns(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetChannel::set_arns): <p>Array of ARNs, one per channel.</p>
                            /// - On success, responds with [`BatchGetChannelOutput`](crate::output::BatchGetChannelOutput) with field(s):
    ///   - [`channels(Option<Vec<Channel>>)`](crate::output::BatchGetChannelOutput::channels): <p></p>
    ///   - [`errors(Option<Vec<BatchError>>)`](crate::output::BatchGetChannelOutput::errors): <p>Each error object is related to a specific ARN in the request.</p>
                            /// - On failure, responds with [`SdkError<BatchGetChannelError>`](crate::error::BatchGetChannelError)
    pub fn batch_get_channel(&self) -> crate::client::fluent_builders::BatchGetChannel {
                                crate::client::fluent_builders::BatchGetChannel::new(self.handle.clone())
                            }
}

