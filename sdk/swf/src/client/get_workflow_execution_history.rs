// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWorkflowExecutionHistory`](crate::client::fluent_builders::GetWorkflowExecutionHistory) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetWorkflowExecutionHistory::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::set_domain): <p>The name of the domain containing the workflow execution.</p>
    ///   - [`execution(WorkflowExecution)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::execution) / [`set_execution(Option<WorkflowExecution>)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::set_execution): <p>Specifies the workflow execution for which to return the history.</p>
    ///   - [`next_page_token(impl Into<String>)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::next_page_token) / [`set_next_page_token(Option<String>)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::set_next_page_token): <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>  <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
    ///   - [`maximum_page_size(i32)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::maximum_page_size) / [`set_maximum_page_size(i32)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::set_maximum_page_size): <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    ///   - [`reverse_order(bool)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::reverse_order) / [`set_reverse_order(bool)`](crate::client::fluent_builders::GetWorkflowExecutionHistory::set_reverse_order): <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimeStamp</code> of the events.</p>
                            /// - On success, responds with [`GetWorkflowExecutionHistoryOutput`](crate::output::GetWorkflowExecutionHistoryOutput) with field(s):
    ///   - [`events(Option<Vec<HistoryEvent>>)`](crate::output::GetWorkflowExecutionHistoryOutput::events): <p>The list of history events.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::GetWorkflowExecutionHistoryOutput::next_page_token): <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>  <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
                            /// - On failure, responds with [`SdkError<GetWorkflowExecutionHistoryError>`](crate::error::GetWorkflowExecutionHistoryError)
    pub fn get_workflow_execution_history(&self) -> crate::client::fluent_builders::GetWorkflowExecutionHistory {
                                crate::client::fluent_builders::GetWorkflowExecutionHistory::new(self.handle.clone())
                            }
}

