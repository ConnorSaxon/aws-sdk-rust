// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRecommenderConfigurations`](crate::client::fluent_builders::GetRecommenderConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`page_size(impl Into<String>)`](crate::client::fluent_builders::GetRecommenderConfigurations::page_size) / [`set_page_size(Option<String>)`](crate::client::fluent_builders::GetRecommenderConfigurations::set_page_size): <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    ///   - [`token(impl Into<String>)`](crate::client::fluent_builders::GetRecommenderConfigurations::token) / [`set_token(Option<String>)`](crate::client::fluent_builders::GetRecommenderConfigurations::set_token): <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
                            /// - On success, responds with [`GetRecommenderConfigurationsOutput`](crate::output::GetRecommenderConfigurationsOutput) with field(s):
    ///   - [`list_recommender_configurations_response(Option<ListRecommenderConfigurationsResponse>)`](crate::output::GetRecommenderConfigurationsOutput::list_recommender_configurations_response): <p>Provides information about all the recommender model configurations that are associated with your Amazon Pinpoint account.</p>
                            /// - On failure, responds with [`SdkError<GetRecommenderConfigurationsError>`](crate::error::GetRecommenderConfigurationsError)
    pub fn get_recommender_configurations(&self) -> crate::client::fluent_builders::GetRecommenderConfigurations {
                                crate::client::fluent_builders::GetRecommenderConfigurations::new(self.handle.clone())
                            }
}

