// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRealtimeContactAnalysisSegments`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_instance_id): <p>The identifier of the Amazon Connect instance.</p>
    ///   - [`contact_id(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::contact_id) / [`set_contact_id(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_contact_id): <p>The identifier of the contact.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_max_results): <p>The maximimum number of results to return per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
                            /// - On success, responds with [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput) with field(s):
    ///   - [`segments(Option<Vec<RealtimeContactAnalysisSegment>>)`](crate::output::ListRealtimeContactAnalysisSegmentsOutput::segments): <p>An analyzed transcript or category.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRealtimeContactAnalysisSegmentsOutput::next_token): <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p>  <ul>   <li> <p>There are more segments so another call is required to get them.</p> </li>   <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li>  </ul>  <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
                            /// - On failure, responds with [`SdkError<ListRealtimeContactAnalysisSegmentsError>`](crate::error::ListRealtimeContactAnalysisSegmentsError)
    pub fn list_realtime_contact_analysis_segments(&self) -> crate::client::fluent_builders::ListRealtimeContactAnalysisSegments {
                                crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::new(self.handle.clone())
                            }
}

