// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateStreamingDistribution`](crate::client::fluent_builders::UpdateStreamingDistribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`streaming_distribution_config(StreamingDistributionConfig)`](crate::client::fluent_builders::UpdateStreamingDistribution::streaming_distribution_config) / [`set_streaming_distribution_config(Option<StreamingDistributionConfig>)`](crate::client::fluent_builders::UpdateStreamingDistribution::set_streaming_distribution_config): <p>The streaming distribution's configuration information.</p>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateStreamingDistribution::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateStreamingDistribution::set_id): <p>The streaming distribution's id.</p>
    ///   - [`if_match(impl Into<String>)`](crate::client::fluent_builders::UpdateStreamingDistribution::if_match) / [`set_if_match(Option<String>)`](crate::client::fluent_builders::UpdateStreamingDistribution::set_if_match): <p>The value of the <code>ETag</code> header that you received when retrieving the streaming distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
                            /// - On success, responds with [`UpdateStreamingDistributionOutput`](crate::output::UpdateStreamingDistributionOutput) with field(s):
    ///   - [`streaming_distribution(Option<StreamingDistribution>)`](crate::output::UpdateStreamingDistributionOutput::streaming_distribution): <p>The streaming distribution's information.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::UpdateStreamingDistributionOutput::e_tag): <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
                            /// - On failure, responds with [`SdkError<UpdateStreamingDistributionError>`](crate::error::UpdateStreamingDistributionError)
    pub fn update_streaming_distribution(&self) -> crate::client::fluent_builders::UpdateStreamingDistribution {
                                crate::client::fluent_builders::UpdateStreamingDistribution::new(self.handle.clone())
                            }
}

