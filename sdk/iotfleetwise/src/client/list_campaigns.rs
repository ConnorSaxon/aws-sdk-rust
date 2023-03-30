// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCampaigns`](crate::client::fluent_builders::ListCampaigns) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCampaigns::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCampaigns::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCampaigns::set_next_token): <p>A pagination token for the next set of results.</p>  <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next set of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCampaigns::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCampaigns::set_max_results): <p> The maximum number of items to return, between 1 and 100, inclusive. </p>
    ///   - [`status(impl Into<String>)`](crate::client::fluent_builders::ListCampaigns::status) / [`set_status(Option<String>)`](crate::client::fluent_builders::ListCampaigns::set_status): <p>Optional parameter to filter the results by the status of each created campaign in your account. The status can be one of: <code>CREATING</code>, <code>WAITING_FOR_APPROVAL</code>, <code>RUNNING</code>, or <code>SUSPENDED</code>.</p>
                            /// - On success, responds with [`ListCampaignsOutput`](crate::output::ListCampaignsOutput) with field(s):
    ///   - [`campaign_summaries(Option<Vec<CampaignSummary>>)`](crate::output::ListCampaignsOutput::campaign_summaries): <p> A summary of information about each campaign. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListCampaignsOutput::next_token): <p> The token to retrieve the next set of results, or <code>null</code> if there are no more results. </p>
                            /// - On failure, responds with [`SdkError<ListCampaignsError>`](crate::error::ListCampaignsError)
    pub fn list_campaigns(&self) -> crate::client::fluent_builders::ListCampaigns {
                                crate::client::fluent_builders::ListCampaigns::new(self.handle.clone())
                            }
}

