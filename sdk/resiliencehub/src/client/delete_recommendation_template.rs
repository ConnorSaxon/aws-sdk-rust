// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRecommendationTemplate`](crate::client::fluent_builders::DeleteRecommendationTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recommendation_template_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteRecommendationTemplate::recommendation_template_arn) / [`set_recommendation_template_arn(Option<String>)`](crate::client::fluent_builders::DeleteRecommendationTemplate::set_recommendation_template_arn): <p>The Amazon Resource Name (ARN) for a recommendation template.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::DeleteRecommendationTemplate::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::DeleteRecommendationTemplate::set_client_token): <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. You should not reuse the same client token for other API requests.</p>
                            /// - On success, responds with [`DeleteRecommendationTemplateOutput`](crate::output::DeleteRecommendationTemplateOutput) with field(s):
    ///   - [`recommendation_template_arn(Option<String>)`](crate::output::DeleteRecommendationTemplateOutput::recommendation_template_arn): <p>The Amazon Resource Name (ARN) for a recommendation template.</p>
    ///   - [`status(Option<RecommendationTemplateStatus>)`](crate::output::DeleteRecommendationTemplateOutput::status): <p>The status of the action.</p>
                            /// - On failure, responds with [`SdkError<DeleteRecommendationTemplateError>`](crate::error::DeleteRecommendationTemplateError)
    pub fn delete_recommendation_template(&self) -> crate::client::fluent_builders::DeleteRecommendationTemplate {
                                crate::client::fluent_builders::DeleteRecommendationTemplate::new(self.handle.clone())
                            }
}

