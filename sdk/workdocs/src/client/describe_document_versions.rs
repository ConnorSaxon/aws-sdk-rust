// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDocumentVersions`](crate::client::fluent_builders::DescribeDocumentVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeDocumentVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::authentication_token) / [`set_authentication_token(Option<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_authentication_token): <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    ///   - [`document_id(impl Into<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::document_id) / [`set_document_id(Option<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_document_id): <p>The ID of the document.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_marker): <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeDocumentVersions::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_limit): <p>The maximum number of versions to return with this call.</p>
    ///   - [`include(impl Into<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::include) / [`set_include(Option<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_include): <p>A comma-separated list of values. Specify "INITIALIZED" to include incomplete versions.</p>
    ///   - [`fields(impl Into<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::fields) / [`set_fields(Option<String>)`](crate::client::fluent_builders::DescribeDocumentVersions::set_fields): <p>Specify "SOURCE" to include initialized versions and a URL for the source document.</p>
                            /// - On success, responds with [`DescribeDocumentVersionsOutput`](crate::output::DescribeDocumentVersionsOutput) with field(s):
    ///   - [`document_versions(Option<Vec<DocumentVersionMetadata>>)`](crate::output::DescribeDocumentVersionsOutput::document_versions): <p>The document versions.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeDocumentVersionsOutput::marker): <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
                            /// - On failure, responds with [`SdkError<DescribeDocumentVersionsError>`](crate::error::DescribeDocumentVersionsError)
    pub fn describe_document_versions(&self) -> crate::client::fluent_builders::DescribeDocumentVersions {
                                crate::client::fluent_builders::DescribeDocumentVersions::new(self.handle.clone())
                            }
}

