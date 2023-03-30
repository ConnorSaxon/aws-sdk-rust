// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListJobs`](crate::client::fluent_builders::ListJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListJobs::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListJobs::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    ///   - [`vault_name(impl Into<String>)`](crate::client::fluent_builders::ListJobs::vault_name) / [`set_vault_name(Option<String>)`](crate::client::fluent_builders::ListJobs::set_vault_name): <p>The name of the vault.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListJobs::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListJobs::set_limit): <p>The maximum number of jobs to be returned. The default limit is 50. The number of jobs returned might be fewer than the specified limit, but the number of returned jobs never exceeds the limit.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListJobs::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListJobs::set_marker): <p>An opaque string used for pagination. This value specifies the job at which the listing of jobs should begin. Get the marker value from a previous List Jobs response. You only need to include the marker if you are continuing the pagination of results started in a previous List Jobs request.</p>
    ///   - [`statuscode(impl Into<String>)`](crate::client::fluent_builders::ListJobs::statuscode) / [`set_statuscode(Option<String>)`](crate::client::fluent_builders::ListJobs::set_statuscode): <p>The type of job status to return. You can specify the following values: <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code>.</p>
    ///   - [`completed(impl Into<String>)`](crate::client::fluent_builders::ListJobs::completed) / [`set_completed(Option<String>)`](crate::client::fluent_builders::ListJobs::set_completed): <p>The state of the jobs to return. You can specify <code>true</code> or <code>false</code>.</p>
                            /// - On success, responds with [`ListJobsOutput`](crate::output::ListJobsOutput) with field(s):
    ///   - [`job_list(Option<Vec<GlacierJobDescription>>)`](crate::output::ListJobsOutput::job_list): <p>A list of job objects. Each job object contains metadata describing the job.</p>
    ///   - [`marker(Option<String>)`](crate::output::ListJobsOutput::marker): <p> An opaque string used for pagination that specifies the job at which the listing of jobs should begin. You get the <code>marker</code> value from a previous List Jobs response. You only need to include the marker if you are continuing the pagination of the results started in a previous List Jobs request. </p>
                            /// - On failure, responds with [`SdkError<ListJobsError>`](crate::error::ListJobsError)
    pub fn list_jobs(&self) -> crate::client::fluent_builders::ListJobs {
                                crate::client::fluent_builders::ListJobs::new(self.handle.clone())
                            }
}

