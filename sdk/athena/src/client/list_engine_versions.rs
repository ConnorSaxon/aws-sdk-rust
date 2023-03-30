// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEngineVersions`](crate::client::fluent_builders::ListEngineVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEngineVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEngineVersions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEngineVersions::set_next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEngineVersions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListEngineVersions::set_max_results): <p>The maximum number of engine versions to return in this request.</p>
                            /// - On success, responds with [`ListEngineVersionsOutput`](crate::output::ListEngineVersionsOutput) with field(s):
    ///   - [`engine_versions(Option<Vec<EngineVersion>>)`](crate::output::ListEngineVersionsOutput::engine_versions): <p>A list of engine versions that are available to choose from.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListEngineVersionsOutput::next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
                            /// - On failure, responds with [`SdkError<ListEngineVersionsError>`](crate::error::ListEngineVersionsError)
    pub fn list_engine_versions(&self) -> crate::client::fluent_builders::ListEngineVersions {
                                crate::client::fluent_builders::ListEngineVersions::new(self.handle.clone())
                            }
}

