// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListJobs`](crate::client::fluent_builders::ListJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListJobs::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListJobs::set_account_id): <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p>
    ///   - [`job_statuses(Vec<JobStatus>)`](crate::client::fluent_builders::ListJobs::job_statuses) / [`set_job_statuses(Option<Vec<JobStatus>>)`](crate::client::fluent_builders::ListJobs::set_job_statuses): <p>The <code>List Jobs</code> request returns jobs that match the statuses listed in this element.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListJobs::set_next_token): <p>A pagination token to request the next page of results. Use the token that Amazon S3 returned in the <code>NextToken</code> element of the <code>ListJobsResult</code> from the previous <code>List Jobs</code> request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListJobs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListJobs::set_max_results): <p>The maximum number of jobs that Amazon S3 will include in the <code>List Jobs</code> response. If there are more jobs than this number, the response will include a pagination token in the <code>NextToken</code> field to enable you to retrieve the next page of results.</p>
                            /// - On success, responds with [`ListJobsOutput`](crate::output::ListJobsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListJobsOutput::next_token): <p>If the <code>List Jobs</code> request produced more than the maximum number of results, you can pass this value into a subsequent <code>List Jobs</code> request in order to retrieve the next page of results.</p>
    ///   - [`jobs(Option<Vec<JobListDescriptor>>)`](crate::output::ListJobsOutput::jobs): <p>The list of current jobs and jobs that have ended within the last 30 days.</p>
                            /// - On failure, responds with [`SdkError<ListJobsError>`](crate::error::ListJobsError)
    pub fn list_jobs(&self) -> crate::client::fluent_builders::ListJobs {
                                crate::client::fluent_builders::ListJobs::new(self.handle.clone())
                            }
}

