// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateMediaConcatenationPipeline`](crate::client::fluent_builders::CreateMediaConcatenationPipeline) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`sources(Vec<ConcatenationSource>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::sources) / [`set_sources(Option<Vec<ConcatenationSource>>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::set_sources): <p>An object that specifies the sources for the media concatenation pipeline.</p>
    ///   - [`sinks(Vec<ConcatenationSink>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::sinks) / [`set_sinks(Option<Vec<ConcatenationSink>>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::set_sinks): <p>An object that specifies the data sinks for the media concatenation pipeline.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::set_client_request_token): <p>The unique identifier for the client request. The token makes the API request idempotent. Use a unique token for each media concatenation pipeline request.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateMediaConcatenationPipeline::set_tags): <p>The tags associated with the media concatenation pipeline.</p>
                            /// - On success, responds with [`CreateMediaConcatenationPipelineOutput`](crate::output::CreateMediaConcatenationPipelineOutput) with field(s):
    ///   - [`media_concatenation_pipeline(Option<MediaConcatenationPipeline>)`](crate::output::CreateMediaConcatenationPipelineOutput::media_concatenation_pipeline): <p>A media concatenation pipeline object, the ID, source type, <code>MediaPipelineARN</code>, and sink of a media concatenation pipeline object.</p>
                            /// - On failure, responds with [`SdkError<CreateMediaConcatenationPipelineError>`](crate::error::CreateMediaConcatenationPipelineError)
    pub fn create_media_concatenation_pipeline(&self) -> crate::client::fluent_builders::CreateMediaConcatenationPipeline {
                                crate::client::fluent_builders::CreateMediaConcatenationPipeline::new(self.handle.clone())
                            }
}

