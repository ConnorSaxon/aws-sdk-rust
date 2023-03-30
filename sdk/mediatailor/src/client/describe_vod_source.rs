// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeVodSource`](crate::client::fluent_builders::DescribeVodSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_location_name(impl Into<String>)`](crate::client::fluent_builders::DescribeVodSource::source_location_name) / [`set_source_location_name(Option<String>)`](crate::client::fluent_builders::DescribeVodSource::set_source_location_name): <p>The name of the source location associated with this VOD Source.</p>
    ///   - [`vod_source_name(impl Into<String>)`](crate::client::fluent_builders::DescribeVodSource::vod_source_name) / [`set_vod_source_name(Option<String>)`](crate::client::fluent_builders::DescribeVodSource::set_vod_source_name): <p>The name of the VOD Source.</p>
                            /// - On success, responds with [`DescribeVodSourceOutput`](crate::output::DescribeVodSourceOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DescribeVodSourceOutput::arn): <p>The ARN of the VOD source.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeVodSourceOutput::creation_time): <p>The timestamp that indicates when the VOD source was created.</p>
    ///   - [`http_package_configurations(Option<Vec<HttpPackageConfiguration>>)`](crate::output::DescribeVodSourceOutput::http_package_configurations): <p>The HTTP package configurations.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DescribeVodSourceOutput::last_modified_time): <p>The last modified time of the VOD source.</p>
    ///   - [`source_location_name(Option<String>)`](crate::output::DescribeVodSourceOutput::source_location_name): <p>The name of the source location associated with the VOD source.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribeVodSourceOutput::tags): <p>The tags assigned to the VOD source. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/tagging.html">Tagging AWS Elemental MediaTailor Resources</a>.</p>
    ///   - [`vod_source_name(Option<String>)`](crate::output::DescribeVodSourceOutput::vod_source_name): <p>The name of the VOD source.</p>
                            /// - On failure, responds with [`SdkError<DescribeVodSourceError>`](crate::error::DescribeVodSourceError)
    pub fn describe_vod_source(&self) -> crate::client::fluent_builders::DescribeVodSource {
                                crate::client::fluent_builders::DescribeVodSource::new(self.handle.clone())
                            }
}

