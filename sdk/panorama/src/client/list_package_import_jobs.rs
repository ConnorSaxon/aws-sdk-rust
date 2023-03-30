// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPackageImportJobs`](crate::client::fluent_builders::ListPackageImportJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPackageImportJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPackageImportJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPackageImportJobs::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPackageImportJobs::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListPackageImportJobs::set_max_results): <p>The maximum number of package import jobs to return in one page of results.</p>
                            /// - On success, responds with [`ListPackageImportJobsOutput`](crate::output::ListPackageImportJobsOutput) with field(s):
    ///   - [`package_import_jobs(Option<Vec<PackageImportJob>>)`](crate::output::ListPackageImportJobsOutput::package_import_jobs): <p>A list of package import jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPackageImportJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
                            /// - On failure, responds with [`SdkError<ListPackageImportJobsError>`](crate::error::ListPackageImportJobsError)
    pub fn list_package_import_jobs(&self) -> crate::client::fluent_builders::ListPackageImportJobs {
                                crate::client::fluent_builders::ListPackageImportJobs::new(self.handle.clone())
                            }
}

