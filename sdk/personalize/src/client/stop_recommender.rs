// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopRecommender`](crate::client::fluent_builders::StopRecommender) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recommender_arn(impl Into<String>)`](crate::client::fluent_builders::StopRecommender::recommender_arn) / [`set_recommender_arn(Option<String>)`](crate::client::fluent_builders::StopRecommender::set_recommender_arn): <p>The Amazon Resource Name (ARN) of the recommender to stop.</p>
                            /// - On success, responds with [`StopRecommenderOutput`](crate::output::StopRecommenderOutput) with field(s):
    ///   - [`recommender_arn(Option<String>)`](crate::output::StopRecommenderOutput::recommender_arn): <p>The Amazon Resource Name (ARN) of the recommender you stopped.</p>
                            /// - On failure, responds with [`SdkError<StopRecommenderError>`](crate::error::StopRecommenderError)
    pub fn stop_recommender(&self) -> crate::client::fluent_builders::StopRecommender {
                                crate::client::fluent_builders::StopRecommender::new(self.handle.clone())
                            }
}

