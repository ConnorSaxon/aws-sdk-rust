// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchDatabasesByLFTags`](crate::client::fluent_builders::SearchDatabasesByLFTags) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchDatabasesByLFTags::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::set_next_token): <p>A continuation token, if this is not the first call to retrieve this list.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchDatabasesByLFTags::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::set_catalog_id): <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    ///   - [`expression(Vec<LfTag>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::expression) / [`set_expression(Option<Vec<LfTag>>)`](crate::client::fluent_builders::SearchDatabasesByLFTags::set_expression): <p>A list of conditions (<code>LFTag</code> structures) to search for in database resources.</p>
                            /// - On success, responds with [`SearchDatabasesByLfTagsOutput`](crate::output::SearchDatabasesByLfTagsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::SearchDatabasesByLfTagsOutput::next_token): <p>A continuation token, present if the current list segment is not the last.</p>
    ///   - [`database_list(Option<Vec<TaggedDatabase>>)`](crate::output::SearchDatabasesByLfTagsOutput::database_list): <p>A list of databases that meet the LF-tag conditions.</p>
                            /// - On failure, responds with [`SdkError<SearchDatabasesByLFTagsError>`](crate::error::SearchDatabasesByLFTagsError)
    pub fn search_databases_by_lf_tags(&self) -> crate::client::fluent_builders::SearchDatabasesByLFTags {
                                crate::client::fluent_builders::SearchDatabasesByLFTags::new(self.handle.clone())
                            }
}

