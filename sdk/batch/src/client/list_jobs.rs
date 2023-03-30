// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListJobs`](crate::client::fluent_builders::ListJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_queue(impl Into<String>)`](crate::client::fluent_builders::ListJobs::job_queue) / [`set_job_queue(Option<String>)`](crate::client::fluent_builders::ListJobs::set_job_queue): <p>The name or full Amazon Resource Name (ARN) of the job queue used to list jobs.</p>
    ///   - [`array_job_id(impl Into<String>)`](crate::client::fluent_builders::ListJobs::array_job_id) / [`set_array_job_id(Option<String>)`](crate::client::fluent_builders::ListJobs::set_array_job_id): <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    ///   - [`multi_node_job_id(impl Into<String>)`](crate::client::fluent_builders::ListJobs::multi_node_job_id) / [`set_multi_node_job_id(Option<String>)`](crate::client::fluent_builders::ListJobs::set_multi_node_job_id): <p>The job ID for a multi-node parallel job. Specifying a multi-node parallel job ID with this parameter lists all nodes that are associated with the specified job.</p>
    ///   - [`job_status(JobStatus)`](crate::client::fluent_builders::ListJobs::job_status) / [`set_job_status(Option<JobStatus>)`](crate::client::fluent_builders::ListJobs::set_job_status): <p>The job status used to filter jobs in the specified queue. If the <code>filters</code> parameter is specified, the <code>jobStatus</code> parameter is ignored and jobs with any status are returned. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListJobs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListJobs::set_max_results): <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListJobs::set_next_token): <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note>   <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    ///   - [`filters(Vec<KeyValuesPair>)`](crate::client::fluent_builders::ListJobs::filters) / [`set_filters(Option<Vec<KeyValuesPair>>)`](crate::client::fluent_builders::ListJobs::set_filters): <p>The filter to apply to the query. Only one filter can be used at a time. When the filter is used, <code>jobStatus</code> is ignored. The filter doesn't apply to child jobs in an array or multi-node parallel (MNP) jobs. The results are sorted by the <code>createdAt</code> field, with the most recent jobs being first.</p>  <dl>   <dt>   JOB_NAME  </dt>   <dd>    <p>The value of the filter is a case-insensitive match for the job name. If the value ends with an asterisk (*), the filter matches any job name that begins with the string before the '*'. This corresponds to the <code>jobName</code> value. For example, <code>test1</code> matches both <code>Test1</code> and <code>test1</code>, and <code>test1*</code> matches both <code>test1</code> and <code>Test10</code>. When the <code>JOB_NAME</code> filter is used, the results are grouped by the job name and version.</p>   </dd>   <dt>   JOB_DEFINITION  </dt>   <dd>    <p>The value for the filter is the name or Amazon Resource Name (ARN) of the job definition. This corresponds to the <code>jobDefinition</code> value. The value is case sensitive. When the value for the filter is the job definition name, the results include all the jobs that used any revision of that job definition name. If the value ends with an asterisk (*), the filter matches any job definition name that begins with the string before the '*'. For example, <code>jd1</code> matches only <code>jd1</code>, and <code>jd1*</code> matches both <code>jd1</code> and <code>jd1A</code>. The version of the job definition that's used doesn't affect the sort order. When the <code>JOB_DEFINITION</code> filter is used and the ARN is used (which is in the form <code>arn:${Partition}:batch:${Region}:${Account}:job-definition/${JobDefinitionName}:${Revision}</code>), the results include jobs that used the specified revision of the job definition. Asterisk (*) isn't supported when the ARN is used.</p>   </dd>   <dt>   BEFORE_CREATED_AT  </dt>   <dd>    <p>The value for the filter is the time that's before the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>   </dd>   <dt>   AFTER_CREATED_AT  </dt>   <dd>    <p>The value for the filter is the time that's after the job was created. This corresponds to the <code>createdAt</code> value. The value is a string representation of the number of milliseconds since 00:00:00 UTC (midnight) on January 1, 1970.</p>   </dd>  </dl>
                            /// - On success, responds with [`ListJobsOutput`](crate::output::ListJobsOutput) with field(s):
    ///   - [`job_summary_list(Option<Vec<JobSummary>>)`](crate::output::ListJobsOutput::job_summary_list): <p>A list of job summaries that match the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListJobsOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListJobsError>`](crate::error::ListJobsError)
    pub fn list_jobs(&self) -> crate::client::fluent_builders::ListJobs {
                                crate::client::fluent_builders::ListJobs::new(self.handle.clone())
                            }
}

