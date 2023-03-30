// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSagemakerServicecatalogPortfolioStatus`](crate::client::fluent_builders::GetSagemakerServicecatalogPortfolioStatus) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetSagemakerServicecatalogPortfolioStatus::send) it.
                            /// - On success, responds with [`GetSagemakerServicecatalogPortfolioStatusOutput`](crate::output::GetSagemakerServicecatalogPortfolioStatusOutput) with field(s):
    ///   - [`status(Option<SagemakerServicecatalogStatus>)`](crate::output::GetSagemakerServicecatalogPortfolioStatusOutput::status): <p>Whether Service Catalog is enabled or disabled in SageMaker.</p>
                            /// - On failure, responds with [`SdkError<GetSagemakerServicecatalogPortfolioStatusError>`](crate::error::GetSagemakerServicecatalogPortfolioStatusError)
    pub fn get_sagemaker_servicecatalog_portfolio_status(&self) -> crate::client::fluent_builders::GetSagemakerServicecatalogPortfolioStatus {
                                crate::client::fluent_builders::GetSagemakerServicecatalogPortfolioStatus::new(self.handle.clone())
                            }
}

