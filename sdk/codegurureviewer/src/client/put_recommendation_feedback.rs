// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutRecommendationFeedback`](crate::client::fluent_builders::PutRecommendationFeedback) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`code_review_arn(impl Into<String>)`](crate::client::fluent_builders::PutRecommendationFeedback::code_review_arn) / [`set_code_review_arn(Option<String>)`](crate::client::fluent_builders::PutRecommendationFeedback::set_code_review_arn): <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html">CodeReview</a> object. </p>
    ///   - [`recommendation_id(impl Into<String>)`](crate::client::fluent_builders::PutRecommendationFeedback::recommendation_id) / [`set_recommendation_id(Option<String>)`](crate::client::fluent_builders::PutRecommendationFeedback::set_recommendation_id): <p>The recommendation ID that can be used to track the provided recommendations and then to collect the feedback.</p>
    ///   - [`reactions(Vec<Reaction>)`](crate::client::fluent_builders::PutRecommendationFeedback::reactions) / [`set_reactions(Option<Vec<Reaction>>)`](crate::client::fluent_builders::PutRecommendationFeedback::set_reactions): <p>List for storing reactions. Reactions are utf-8 text code for emojis. If you send an empty list it clears all your feedback.</p>
                            /// - On success, responds with [`PutRecommendationFeedbackOutput`](crate::output::PutRecommendationFeedbackOutput)
                            /// - On failure, responds with [`SdkError<PutRecommendationFeedbackError>`](crate::error::PutRecommendationFeedbackError)
    pub fn put_recommendation_feedback(&self) -> crate::client::fluent_builders::PutRecommendationFeedback {
                                crate::client::fluent_builders::PutRecommendationFeedback::new(self.handle.clone())
                            }
}

