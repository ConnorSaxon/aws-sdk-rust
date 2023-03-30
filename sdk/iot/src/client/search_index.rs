// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchIndex`](crate::client::fluent_builders::SearchIndex) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_name(impl Into<String>)`](crate::client::fluent_builders::SearchIndex::index_name) / [`set_index_name(Option<String>)`](crate::client::fluent_builders::SearchIndex::set_index_name): <p>The search index name.</p>
    ///   - [`query_string(impl Into<String>)`](crate::client::fluent_builders::SearchIndex::query_string) / [`set_query_string(Option<String>)`](crate::client::fluent_builders::SearchIndex::set_query_string): <p>The search query string. For more information about the search query syntax, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query syntax</a>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchIndex::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchIndex::set_next_token): <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchIndex::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchIndex::set_max_results): <p>The maximum number of results to return at one time.</p>
    ///   - [`query_version(impl Into<String>)`](crate::client::fluent_builders::SearchIndex::query_version) / [`set_query_version(Option<String>)`](crate::client::fluent_builders::SearchIndex::set_query_version): <p>The query version.</p>
                            /// - On success, responds with [`SearchIndexOutput`](crate::output::SearchIndexOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::SearchIndexOutput::next_token): <p>The token used to get the next set of results, or <code>null</code> if there are no additional results.</p>
    ///   - [`things(Option<Vec<ThingDocument>>)`](crate::output::SearchIndexOutput::things): <p>The things that match the search query.</p>
    ///   - [`thing_groups(Option<Vec<ThingGroupDocument>>)`](crate::output::SearchIndexOutput::thing_groups): <p>The thing groups that match the search query.</p>
                            /// - On failure, responds with [`SdkError<SearchIndexError>`](crate::error::SearchIndexError)
    pub fn search_index(&self) -> crate::client::fluent_builders::SearchIndex {
                                crate::client::fluent_builders::SearchIndex::new(self.handle.clone())
                            }
}

