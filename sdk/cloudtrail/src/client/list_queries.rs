// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListQueries`](crate::client::fluent_builders::ListQueries) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListQueries::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_data_store(impl Into<String>)`](crate::client::fluent_builders::ListQueries::event_data_store) / [`set_event_data_store(Option<String>)`](crate::client::fluent_builders::ListQueries::set_event_data_store): <p>The ARN (or the ID suffix of the ARN) of an event data store on which queries were run.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListQueries::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListQueries::set_next_token): <p>A token you can use to get the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListQueries::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListQueries::set_max_results): <p>The maximum number of queries to show on a page.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::ListQueries::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::ListQueries::set_start_time): <p>Use with <code>EndTime</code> to bound a <code>ListQueries</code> request, and limit its results to only those queries run within a specified time period.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::ListQueries::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::ListQueries::set_end_time): <p>Use with <code>StartTime</code> to bound a <code>ListQueries</code> request, and limit its results to only those queries run within a specified time period.</p>
    ///   - [`query_status(QueryStatus)`](crate::client::fluent_builders::ListQueries::query_status) / [`set_query_status(Option<QueryStatus>)`](crate::client::fluent_builders::ListQueries::set_query_status): <p>The status of queries that you want to return in results. Valid values for <code>QueryStatus</code> include <code>QUEUED</code>, <code>RUNNING</code>, <code>FINISHED</code>, <code>FAILED</code>, <code>TIMED_OUT</code>, or <code>CANCELLED</code>.</p>
                            /// - On success, responds with [`ListQueriesOutput`](crate::output::ListQueriesOutput) with field(s):
    ///   - [`queries(Option<Vec<Query>>)`](crate::output::ListQueriesOutput::queries): <p>Lists matching query results, and shows query ID, status, and creation time of each query.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListQueriesOutput::next_token): <p>A token you can use to get the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListQueriesError>`](crate::error::ListQueriesError)
    pub fn list_queries(&self) -> crate::client::fluent_builders::ListQueries {
                                crate::client::fluent_builders::ListQueries::new(self.handle.clone())
                            }
}

