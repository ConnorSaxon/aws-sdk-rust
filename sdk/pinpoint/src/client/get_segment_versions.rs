// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSegmentVersions`](crate::client::fluent_builders::GetSegmentVersions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetSegmentVersions::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetSegmentVersions::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`page_size(impl Into<String>)`](crate::client::fluent_builders::GetSegmentVersions::page_size) / [`set_page_size(Option<String>)`](crate::client::fluent_builders::GetSegmentVersions::set_page_size): <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    ///   - [`segment_id(impl Into<String>)`](crate::client::fluent_builders::GetSegmentVersions::segment_id) / [`set_segment_id(Option<String>)`](crate::client::fluent_builders::GetSegmentVersions::set_segment_id): <p>The unique identifier for the segment.</p>
    ///   - [`token(impl Into<String>)`](crate::client::fluent_builders::GetSegmentVersions::token) / [`set_token(Option<String>)`](crate::client::fluent_builders::GetSegmentVersions::set_token): <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
                            /// - On success, responds with [`GetSegmentVersionsOutput`](crate::output::GetSegmentVersionsOutput) with field(s):
    ///   - [`segments_response(Option<SegmentsResponse>)`](crate::output::GetSegmentVersionsOutput::segments_response): <p>Provides information about all the segments that are associated with an application.</p>
                            /// - On failure, responds with [`SdkError<GetSegmentVersionsError>`](crate::error::GetSegmentVersionsError)
    pub fn get_segment_versions(&self) -> crate::client::fluent_builders::GetSegmentVersions {
                                crate::client::fluent_builders::GetSegmentVersions::new(self.handle.clone())
                            }
}

