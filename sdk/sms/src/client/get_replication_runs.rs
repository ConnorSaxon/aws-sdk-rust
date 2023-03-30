// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetReplicationRuns`](crate::client::fluent_builders::GetReplicationRuns) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetReplicationRuns::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`replication_job_id(impl Into<String>)`](crate::client::fluent_builders::GetReplicationRuns::replication_job_id) / [`set_replication_job_id(Option<String>)`](crate::client::fluent_builders::GetReplicationRuns::set_replication_job_id): <p>The ID of the replication job.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetReplicationRuns::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetReplicationRuns::set_next_token): <p>The token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetReplicationRuns::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetReplicationRuns::set_max_results): <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
                            /// - On success, responds with [`GetReplicationRunsOutput`](crate::output::GetReplicationRunsOutput) with field(s):
    ///   - [`replication_job(Option<ReplicationJob>)`](crate::output::GetReplicationRunsOutput::replication_job): <p>Information about the replication job.</p>
    ///   - [`replication_run_list(Option<Vec<ReplicationRun>>)`](crate::output::GetReplicationRunsOutput::replication_run_list): <p>Information about the replication runs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetReplicationRunsOutput::next_token): <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<GetReplicationRunsError>`](crate::error::GetReplicationRunsError)
    pub fn get_replication_runs(&self) -> crate::client::fluent_builders::GetReplicationRuns {
                                crate::client::fluent_builders::GetReplicationRuns::new(self.handle.clone())
                            }
}

