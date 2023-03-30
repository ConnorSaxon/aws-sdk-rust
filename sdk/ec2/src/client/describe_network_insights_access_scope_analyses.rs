// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeNetworkInsightsAccessScopeAnalyses`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_insights_access_scope_analysis_ids(Vec<String>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::network_insights_access_scope_analysis_ids) / [`set_network_insights_access_scope_analysis_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_network_insights_access_scope_analysis_ids): <p>The IDs of the Network Access Scope analyses.</p>
    ///   - [`network_insights_access_scope_id(impl Into<String>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::network_insights_access_scope_id) / [`set_network_insights_access_scope_id(Option<String>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_network_insights_access_scope_id): <p>The ID of the Network Access Scope.</p>
    ///   - [`analysis_start_time_begin(DateTime)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::analysis_start_time_begin) / [`set_analysis_start_time_begin(Option<DateTime>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_analysis_start_time_begin): <p>Filters the results based on the start time. The analysis must have started on or after this time.</p>
    ///   - [`analysis_start_time_end(DateTime)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::analysis_start_time_end) / [`set_analysis_start_time_end(Option<DateTime>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_analysis_start_time_end): <p>Filters the results based on the start time. The analysis must have started on or before this time.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_filters): <p>There are no supported filters.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`DescribeNetworkInsightsAccessScopeAnalysesOutput`](crate::output::DescribeNetworkInsightsAccessScopeAnalysesOutput) with field(s):
    ///   - [`network_insights_access_scope_analyses(Option<Vec<NetworkInsightsAccessScopeAnalysis>>)`](crate::output::DescribeNetworkInsightsAccessScopeAnalysesOutput::network_insights_access_scope_analyses): <p>The Network Access Scope analyses.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeNetworkInsightsAccessScopeAnalysesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>`](crate::error::DescribeNetworkInsightsAccessScopeAnalysesError)
    pub fn describe_network_insights_access_scope_analyses(&self) -> crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses {
                                crate::client::fluent_builders::DescribeNetworkInsightsAccessScopeAnalyses::new(self.handle.clone())
                            }
}

