// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListVersions`](crate::client::fluent_builders::ListVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListVersions::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListVersions::set_max_results): <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListVersions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListVersions::set_next_token): <p>If your initial <code>ListVersions</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListVersions</code> operations, which returns results in the next page.</p>
                            /// - On success, responds with [`ListVersionsOutput`](crate::output::ListVersionsOutput) with field(s):
    ///   - [`versions(Option<Vec<String>>)`](crate::output::ListVersionsOutput::versions): <p>A list of all versions of OpenSearch and Elasticsearch that Amazon OpenSearch Service supports.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListVersionsOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
                            /// - On failure, responds with [`SdkError<ListVersionsError>`](crate::error::ListVersionsError)
    pub fn list_versions(&self) -> crate::client::fluent_builders::ListVersions {
                                crate::client::fluent_builders::ListVersions::new(self.handle.clone())
                            }
}

