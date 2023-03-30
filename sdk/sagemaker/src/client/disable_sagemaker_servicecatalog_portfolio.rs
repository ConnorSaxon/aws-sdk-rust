// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableSagemakerServicecatalogPortfolio`](crate::client::fluent_builders::DisableSagemakerServicecatalogPortfolio) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DisableSagemakerServicecatalogPortfolio::send) it.
                            /// - On success, responds with [`DisableSagemakerServicecatalogPortfolioOutput`](crate::output::DisableSagemakerServicecatalogPortfolioOutput)
                            /// - On failure, responds with [`SdkError<DisableSagemakerServicecatalogPortfolioError>`](crate::error::DisableSagemakerServicecatalogPortfolioError)
    pub fn disable_sagemaker_servicecatalog_portfolio(&self) -> crate::client::fluent_builders::DisableSagemakerServicecatalogPortfolio {
                                crate::client::fluent_builders::DisableSagemakerServicecatalogPortfolio::new(self.handle.clone())
                            }
}

