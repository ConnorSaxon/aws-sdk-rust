// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListKeywordsForDataSource`](crate::client::fluent_builders::ListKeywordsForDataSource) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListKeywordsForDataSource::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source(SourceType)`](crate::client::fluent_builders::ListKeywordsForDataSource::source) / [`set_source(Option<SourceType>)`](crate::client::fluent_builders::ListKeywordsForDataSource::set_source): <p> The control mapping data source that the keywords apply to. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListKeywordsForDataSource::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListKeywordsForDataSource::set_next_token): <p> The pagination token that's used to fetch the next set of results. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListKeywordsForDataSource::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListKeywordsForDataSource::set_max_results): <p> Represents the maximum number of results on a page or for an API request call. </p>
                            /// - On success, responds with [`ListKeywordsForDataSourceOutput`](crate::output::ListKeywordsForDataSourceOutput) with field(s):
    ///   - [`keywords(Option<Vec<String>>)`](crate::output::ListKeywordsForDataSourceOutput::keywords): <p> The list of keywords for the event mapping source. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListKeywordsForDataSourceOutput::next_token): <p> The pagination token that's used to fetch the next set of results. </p>
                            /// - On failure, responds with [`SdkError<ListKeywordsForDataSourceError>`](crate::error::ListKeywordsForDataSourceError)
    pub fn list_keywords_for_data_source(&self) -> crate::client::fluent_builders::ListKeywordsForDataSource {
                                crate::client::fluent_builders::ListKeywordsForDataSource::new(self.handle.clone())
                            }
}

