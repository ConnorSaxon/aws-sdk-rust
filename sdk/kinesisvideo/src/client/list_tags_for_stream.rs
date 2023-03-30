// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForStream`](crate::client::fluent_builders::ListTagsForStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTagsForStream::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTagsForStream::set_next_token): <p>If you specify this parameter and the result of a <code>ListTagsForStream</code> call is truncated, the response includes a token that you can use in the next request to fetch the next batch of tags.</p>
    ///   - [`stream_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForStream::stream_arn) / [`set_stream_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForStream::set_stream_arn): <p>The Amazon Resource Name (ARN) of the stream that you want to list tags for.</p>
    ///   - [`stream_name(impl Into<String>)`](crate::client::fluent_builders::ListTagsForStream::stream_name) / [`set_stream_name(Option<String>)`](crate::client::fluent_builders::ListTagsForStream::set_stream_name): <p>The name of the stream that you want to list tags for.</p>
                            /// - On success, responds with [`ListTagsForStreamOutput`](crate::output::ListTagsForStreamOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListTagsForStreamOutput::next_token): <p>If you specify this parameter and the result of a <code>ListTags</code> call is truncated, the response includes a token that you can use in the next request to fetch the next set of tags.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::ListTagsForStreamOutput::tags): <p>A map of tag keys and values associated with the specified stream.</p>
                            /// - On failure, responds with [`SdkError<ListTagsForStreamError>`](crate::error::ListTagsForStreamError)
    pub fn list_tags_for_stream(&self) -> crate::client::fluent_builders::ListTagsForStream {
                                crate::client::fluent_builders::ListTagsForStream::new(self.handle.clone())
                            }
}

