// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PollForDecisionTask`](crate::client::fluent_builders::PollForDecisionTask) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::PollForDecisionTask::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::PollForDecisionTask::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::PollForDecisionTask::set_domain): <p>The name of the domain containing the task lists to poll.</p>
    ///   - [`task_list(TaskList)`](crate::client::fluent_builders::PollForDecisionTask::task_list) / [`set_task_list(Option<TaskList>)`](crate::client::fluent_builders::PollForDecisionTask::set_task_list): <p>Specifies the task list to poll for decision tasks.</p>  <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    ///   - [`identity(impl Into<String>)`](crate::client::fluent_builders::PollForDecisionTask::identity) / [`set_identity(Option<String>)`](crate::client::fluent_builders::PollForDecisionTask::set_identity): <p>Identity of the decider making the request, which is recorded in the DecisionTaskStarted event in the workflow history. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    ///   - [`next_page_token(impl Into<String>)`](crate::client::fluent_builders::PollForDecisionTask::next_page_token) / [`set_next_page_token(Option<String>)`](crate::client::fluent_builders::PollForDecisionTask::set_next_page_token): <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p>  <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p> <note>   <p>The <code>nextPageToken</code> returned by this action cannot be used with <code>GetWorkflowExecutionHistory</code> to get the next page. You must call <code>PollForDecisionTask</code> again (with the <code>nextPageToken</code>) to retrieve the next page of history records. Calling <code>PollForDecisionTask</code> with a <code>nextPageToken</code> doesn't return a new decision task.</p>  </note>
    ///   - [`maximum_page_size(i32)`](crate::client::fluent_builders::PollForDecisionTask::maximum_page_size) / [`set_maximum_page_size(i32)`](crate::client::fluent_builders::PollForDecisionTask::set_maximum_page_size): <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>  <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    ///   - [`reverse_order(bool)`](crate::client::fluent_builders::PollForDecisionTask::reverse_order) / [`set_reverse_order(bool)`](crate::client::fluent_builders::PollForDecisionTask::set_reverse_order): <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimestamp</code> of the events.</p>
                            /// - On success, responds with [`PollForDecisionTaskOutput`](crate::output::PollForDecisionTaskOutput) with field(s):
    ///   - [`task_token(Option<String>)`](crate::output::PollForDecisionTaskOutput::task_token): <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    ///   - [`started_event_id(i64)`](crate::output::PollForDecisionTaskOutput::started_event_id): <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    ///   - [`workflow_execution(Option<WorkflowExecution>)`](crate::output::PollForDecisionTaskOutput::workflow_execution): <p>The workflow execution for which this decision task was created.</p>
    ///   - [`workflow_type(Option<WorkflowType>)`](crate::output::PollForDecisionTaskOutput::workflow_type): <p>The type of the workflow execution for which this decision task was created.</p>
    ///   - [`events(Option<Vec<HistoryEvent>>)`](crate::output::PollForDecisionTaskOutput::events): <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::PollForDecisionTaskOutput::next_page_token): <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p>  <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    ///   - [`previous_started_event_id(i64)`](crate::output::PollForDecisionTaskOutput::previous_started_event_id): <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
                            /// - On failure, responds with [`SdkError<PollForDecisionTaskError>`](crate::error::PollForDecisionTaskError)
    pub fn poll_for_decision_task(&self) -> crate::client::fluent_builders::PollForDecisionTask {
                                crate::client::fluent_builders::PollForDecisionTask::new(self.handle.clone())
                            }
}

