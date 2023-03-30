// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListReadSetExportJobs`](crate::client::fluent_builders::ListReadSetExportJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListReadSetExportJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`sequence_store_id(impl Into<String>)`](crate::client::fluent_builders::ListReadSetExportJobs::sequence_store_id) / [`set_sequence_store_id(Option<String>)`](crate::client::fluent_builders::ListReadSetExportJobs::set_sequence_store_id): <p>The jobs' sequence store ID.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListReadSetExportJobs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListReadSetExportJobs::set_max_results): <p>The maximum number of jobs to return in one page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListReadSetExportJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListReadSetExportJobs::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`filter(ExportReadSetFilter)`](crate::client::fluent_builders::ListReadSetExportJobs::filter) / [`set_filter(Option<ExportReadSetFilter>)`](crate::client::fluent_builders::ListReadSetExportJobs::set_filter): <p>A filter to apply to the list.</p>
                            /// - On success, responds with [`ListReadSetExportJobsOutput`](crate::output::ListReadSetExportJobsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListReadSetExportJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    ///   - [`export_jobs(Option<Vec<ExportReadSetJobDetail>>)`](crate::output::ListReadSetExportJobsOutput::export_jobs): <p>A list of jobs.</p>
                            /// - On failure, responds with [`SdkError<ListReadSetExportJobsError>`](crate::error::ListReadSetExportJobsError)
    pub fn list_read_set_export_jobs(&self) -> crate::client::fluent_builders::ListReadSetExportJobs {
                                crate::client::fluent_builders::ListReadSetExportJobs::new(self.handle.clone())
                            }
}

