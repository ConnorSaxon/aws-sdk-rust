// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRecommendationFeedback`](crate::client::fluent_builders::ListRecommendationFeedback) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRecommendationFeedback::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::set_next_token): <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRecommendationFeedback::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListRecommendationFeedback::set_max_results): <p>The maximum number of results that are returned per call. The default is 100.</p>
    ///   - [`code_review_arn(impl Into<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::code_review_arn) / [`set_code_review_arn(Option<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::set_code_review_arn): <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html">CodeReview</a> object. </p>
    ///   - [`user_ids(Vec<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::user_ids) / [`set_user_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ListRecommendationFeedback::set_user_ids): <p>An Amazon Web Services user's account ID or Amazon Resource Name (ARN). Use this ID to query the recommendation feedback for a code review from that user.</p>  <p> The <code>UserId</code> is an IAM principal that can be specified as an Amazon Web Services account ID or an Amazon Resource Name (ARN). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#Principal_specifying"> Specifying a Principal</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p>
    ///   - [`recommendation_ids(Vec<String>)`](crate::client::fluent_builders::ListRecommendationFeedback::recommendation_ids) / [`set_recommendation_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ListRecommendationFeedback::set_recommendation_ids): <p>Used to query the recommendation feedback for a given recommendation.</p>
                            /// - On success, responds with [`ListRecommendationFeedbackOutput`](crate::output::ListRecommendationFeedbackOutput) with field(s):
    ///   - [`recommendation_feedback_summaries(Option<Vec<RecommendationFeedbackSummary>>)`](crate::output::ListRecommendationFeedbackOutput::recommendation_feedback_summaries): <p>Recommendation feedback summaries corresponding to the code review ARN.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRecommendationFeedbackOutput::next_token): <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged.</p>
                            /// - On failure, responds with [`SdkError<ListRecommendationFeedbackError>`](crate::error::ListRecommendationFeedbackError)
    pub fn list_recommendation_feedback(&self) -> crate::client::fluent_builders::ListRecommendationFeedback {
                                crate::client::fluent_builders::ListRecommendationFeedback::new(self.handle.clone())
                            }
}

