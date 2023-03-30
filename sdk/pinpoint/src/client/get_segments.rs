// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSegments`](crate::client::fluent_builders::GetSegments) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetSegments::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetSegments::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`page_size(impl Into<String>)`](crate::client::fluent_builders::GetSegments::page_size) / [`set_page_size(Option<String>)`](crate::client::fluent_builders::GetSegments::set_page_size): <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    ///   - [`token(impl Into<String>)`](crate::client::fluent_builders::GetSegments::token) / [`set_token(Option<String>)`](crate::client::fluent_builders::GetSegments::set_token): <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
                            /// - On success, responds with [`GetSegmentsOutput`](crate::output::GetSegmentsOutput) with field(s):
    ///   - [`segments_response(Option<SegmentsResponse>)`](crate::output::GetSegmentsOutput::segments_response): <p>Provides information about all the segments that are associated with an application.</p>
                            /// - On failure, responds with [`SdkError<GetSegmentsError>`](crate::error::GetSegmentsError)
    pub fn get_segments(&self) -> crate::client::fluent_builders::GetSegments {
                                crate::client::fluent_builders::GetSegments::new(self.handle.clone())
                            }
}

