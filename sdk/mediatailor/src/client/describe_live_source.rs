// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLiveSource`](crate::client::fluent_builders::DescribeLiveSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`live_source_name(impl Into<String>)`](crate::client::fluent_builders::DescribeLiveSource::live_source_name) / [`set_live_source_name(Option<String>)`](crate::client::fluent_builders::DescribeLiveSource::set_live_source_name): <p>The name of the live source.</p>
    ///   - [`source_location_name(impl Into<String>)`](crate::client::fluent_builders::DescribeLiveSource::source_location_name) / [`set_source_location_name(Option<String>)`](crate::client::fluent_builders::DescribeLiveSource::set_source_location_name): <p>The name of the source location associated with this Live Source.</p>
                            /// - On success, responds with [`DescribeLiveSourceOutput`](crate::output::DescribeLiveSourceOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DescribeLiveSourceOutput::arn): <p>The ARN of the live source.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeLiveSourceOutput::creation_time): <p>The timestamp that indicates when the live source was created.</p>
    ///   - [`http_package_configurations(Option<Vec<HttpPackageConfiguration>>)`](crate::output::DescribeLiveSourceOutput::http_package_configurations): <p>The HTTP package configurations.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DescribeLiveSourceOutput::last_modified_time): <p>The timestamp that indicates when the live source was modified.</p>
    ///   - [`live_source_name(Option<String>)`](crate::output::DescribeLiveSourceOutput::live_source_name): <p>The name of the live source.</p>
    ///   - [`source_location_name(Option<String>)`](crate::output::DescribeLiveSourceOutput::source_location_name): <p>The name of the source location associated with the live source.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribeLiveSourceOutput::tags): <p>The tags assigned to the live source. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/tagging.html">Tagging AWS Elemental MediaTailor Resources</a>.</p>
                            /// - On failure, responds with [`SdkError<DescribeLiveSourceError>`](crate::error::DescribeLiveSourceError)
    pub fn describe_live_source(&self) -> crate::client::fluent_builders::DescribeLiveSource {
                                crate::client::fluent_builders::DescribeLiveSource::new(self.handle.clone())
                            }
}

