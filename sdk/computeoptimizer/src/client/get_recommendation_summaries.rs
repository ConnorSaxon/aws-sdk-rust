// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRecommendationSummaries`](crate::client::fluent_builders::GetRecommendationSummaries) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetRecommendationSummaries::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::GetRecommendationSummaries::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::GetRecommendationSummaries::set_account_ids): <p>The ID of the Amazon Web Services account for which to return recommendation summaries.</p>  <p>If your account is the management account of an organization, use this parameter to specify the member account for which you want to return recommendation summaries.</p>  <p>Only one account ID can be specified per request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetRecommendationSummaries::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetRecommendationSummaries::set_next_token): <p>The token to advance to the next page of recommendation summaries.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetRecommendationSummaries::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetRecommendationSummaries::set_max_results): <p>The maximum number of recommendation summaries to return with a single request.</p>  <p>To retrieve the remaining results, make another request with the returned <code>nextToken</code> value.</p>
                            /// - On success, responds with [`GetRecommendationSummariesOutput`](crate::output::GetRecommendationSummariesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::GetRecommendationSummariesOutput::next_token): <p>The token to use to advance to the next page of recommendation summaries.</p>  <p>This value is null when there are no more pages of recommendation summaries to return.</p>
    ///   - [`recommendation_summaries(Option<Vec<RecommendationSummary>>)`](crate::output::GetRecommendationSummariesOutput::recommendation_summaries): <p>An array of objects that summarize a recommendation.</p>
                            /// - On failure, responds with [`SdkError<GetRecommendationSummariesError>`](crate::error::GetRecommendationSummariesError)
    pub fn get_recommendation_summaries(&self) -> crate::client::fluent_builders::GetRecommendationSummaries {
                                crate::client::fluent_builders::GetRecommendationSummaries::new(self.handle.clone())
                            }
}

