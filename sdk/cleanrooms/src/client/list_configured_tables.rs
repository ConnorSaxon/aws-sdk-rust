// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListConfiguredTables`](crate::client::fluent_builders::ListConfiguredTables) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListConfiguredTables::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListConfiguredTables::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListConfiguredTables::set_next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListConfiguredTables::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListConfiguredTables::set_max_results): <p>The maximum size of the results that is returned per call.</p>
                            /// - On success, responds with [`ListConfiguredTablesOutput`](crate::output::ListConfiguredTablesOutput) with field(s):
    ///   - [`configured_table_summaries(Option<Vec<ConfiguredTableSummary>>)`](crate::output::ListConfiguredTablesOutput::configured_table_summaries): <p>The configured tables listed by the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListConfiguredTablesOutput::next_token): <p>The token value retrieved from a previous call to access the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListConfiguredTablesError>`](crate::error::ListConfiguredTablesError)
    pub fn list_configured_tables(&self) -> crate::client::fluent_builders::ListConfiguredTables {
                                crate::client::fluent_builders::ListConfiguredTables::new(self.handle.clone())
                            }
}

