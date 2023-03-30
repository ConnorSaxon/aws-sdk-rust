// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRecommenderConfiguration`](crate::client::fluent_builders::UpdateRecommenderConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recommender_id(impl Into<String>)`](crate::client::fluent_builders::UpdateRecommenderConfiguration::recommender_id) / [`set_recommender_id(Option<String>)`](crate::client::fluent_builders::UpdateRecommenderConfiguration::set_recommender_id): <p>The unique identifier for the recommender model configuration. This identifier is displayed as the <b>Recommender ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`update_recommender_configuration(UpdateRecommenderConfigurationShape)`](crate::client::fluent_builders::UpdateRecommenderConfiguration::update_recommender_configuration) / [`set_update_recommender_configuration(Option<UpdateRecommenderConfigurationShape>)`](crate::client::fluent_builders::UpdateRecommenderConfiguration::set_update_recommender_configuration): <p>Specifies Amazon Pinpoint configuration settings for retrieving and processing recommendation data from a recommender model.</p>
                            /// - On success, responds with [`UpdateRecommenderConfigurationOutput`](crate::output::UpdateRecommenderConfigurationOutput) with field(s):
    ///   - [`recommender_configuration_response(Option<RecommenderConfigurationResponse>)`](crate::output::UpdateRecommenderConfigurationOutput::recommender_configuration_response): <p>Provides information about Amazon Pinpoint configuration settings for retrieving and processing data from a recommender model.</p>
                            /// - On failure, responds with [`SdkError<UpdateRecommenderConfigurationError>`](crate::error::UpdateRecommenderConfigurationError)
    pub fn update_recommender_configuration(&self) -> crate::client::fluent_builders::UpdateRecommenderConfiguration {
                                crate::client::fluent_builders::UpdateRecommenderConfiguration::new(self.handle.clone())
                            }
}

