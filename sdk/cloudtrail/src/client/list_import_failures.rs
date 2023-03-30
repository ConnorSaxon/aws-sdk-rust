// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListImportFailures`](crate::client::fluent_builders::ListImportFailures) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListImportFailures::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`import_id(impl Into<String>)`](crate::client::fluent_builders::ListImportFailures::import_id) / [`set_import_id(Option<String>)`](crate::client::fluent_builders::ListImportFailures::set_import_id): <p> The ID of the import. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListImportFailures::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListImportFailures::set_max_results): <p> The maximum number of failures to display on a single page. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListImportFailures::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListImportFailures::set_next_token): <p> A token you can use to get the next page of import failures. </p>
                            /// - On success, responds with [`ListImportFailuresOutput`](crate::output::ListImportFailuresOutput) with field(s):
    ///   - [`failures(Option<Vec<ImportFailureListItem>>)`](crate::output::ListImportFailuresOutput::failures): <p> Contains information about the import failures. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListImportFailuresOutput::next_token): <p> A token you can use to get the next page of results. </p>
                            /// - On failure, responds with [`SdkError<ListImportFailuresError>`](crate::error::ListImportFailuresError)
    pub fn list_import_failures(&self) -> crate::client::fluent_builders::ListImportFailures {
                                crate::client::fluent_builders::ListImportFailures::new(self.handle.clone())
                            }
}

