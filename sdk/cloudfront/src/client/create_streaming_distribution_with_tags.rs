// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateStreamingDistributionWithTags`](crate::client::fluent_builders::CreateStreamingDistributionWithTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`streaming_distribution_config_with_tags(StreamingDistributionConfigWithTags)`](crate::client::fluent_builders::CreateStreamingDistributionWithTags::streaming_distribution_config_with_tags) / [`set_streaming_distribution_config_with_tags(Option<StreamingDistributionConfigWithTags>)`](crate::client::fluent_builders::CreateStreamingDistributionWithTags::set_streaming_distribution_config_with_tags): <p>The streaming distribution's configuration information.</p>
                            /// - On success, responds with [`CreateStreamingDistributionWithTagsOutput`](crate::output::CreateStreamingDistributionWithTagsOutput) with field(s):
    ///   - [`streaming_distribution(Option<StreamingDistribution>)`](crate::output::CreateStreamingDistributionWithTagsOutput::streaming_distribution): <p>The streaming distribution's information.</p>
    ///   - [`location(Option<String>)`](crate::output::CreateStreamingDistributionWithTagsOutput::location): <p>The fully qualified URI of the new streaming distribution resource just created.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::CreateStreamingDistributionWithTagsOutput::e_tag): <p>The current version of the distribution created.</p>
                            /// - On failure, responds with [`SdkError<CreateStreamingDistributionWithTagsError>`](crate::error::CreateStreamingDistributionWithTagsError)
    pub fn create_streaming_distribution_with_tags(&self) -> crate::client::fluent_builders::CreateStreamingDistributionWithTags {
                                crate::client::fluent_builders::CreateStreamingDistributionWithTags::new(self.handle.clone())
                            }
}

