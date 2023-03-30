// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListNotebookExecutions`](crate::client::fluent_builders::ListNotebookExecutions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListNotebookExecutions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`editor_id(impl Into<String>)`](crate::client::fluent_builders::ListNotebookExecutions::editor_id) / [`set_editor_id(Option<String>)`](crate::client::fluent_builders::ListNotebookExecutions::set_editor_id): <p>The unique ID of the editor associated with the notebook execution.</p>
    ///   - [`status(NotebookExecutionStatus)`](crate::client::fluent_builders::ListNotebookExecutions::status) / [`set_status(Option<NotebookExecutionStatus>)`](crate::client::fluent_builders::ListNotebookExecutions::set_status): <p>The status filter for listing notebook executions.</p>  <ul>   <li> <p> <code>START_PENDING</code> indicates that the cluster has received the execution request but execution has not begun.</p> </li>   <li> <p> <code>STARTING</code> indicates that the execution is starting on the cluster.</p> </li>   <li> <p> <code>RUNNING</code> indicates that the execution is being processed by the cluster.</p> </li>   <li> <p> <code>FINISHING</code> indicates that execution processing is in the final stages.</p> </li>   <li> <p> <code>FINISHED</code> indicates that the execution has completed without error.</p> </li>   <li> <p> <code>FAILING</code> indicates that the execution is failing and will not finish successfully.</p> </li>   <li> <p> <code>FAILED</code> indicates that the execution failed.</p> </li>   <li> <p> <code>STOP_PENDING</code> indicates that the cluster has received a <code>StopNotebookExecution</code> request and the stop is pending.</p> </li>   <li> <p> <code>STOPPING</code> indicates that the cluster is in the process of stopping the execution as a result of a <code>StopNotebookExecution</code> request.</p> </li>   <li> <p> <code>STOPPED</code> indicates that the execution stopped because of a <code>StopNotebookExecution</code> request.</p> </li>  </ul>
    ///   - [`from(DateTime)`](crate::client::fluent_builders::ListNotebookExecutions::from) / [`set_from(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookExecutions::set_from): <p>The beginning of time range filter for listing notebook executions. The default is the timestamp of 30 days ago.</p>
    ///   - [`to(DateTime)`](crate::client::fluent_builders::ListNotebookExecutions::to) / [`set_to(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookExecutions::set_to): <p>The end of time range filter for listing notebook executions. The default is the current timestamp.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListNotebookExecutions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListNotebookExecutions::set_marker): <p>The pagination token, returned by a previous <code>ListNotebookExecutions</code> call, that indicates the start of the list for this <code>ListNotebookExecutions</code> call.</p>
                            /// - On success, responds with [`ListNotebookExecutionsOutput`](crate::output::ListNotebookExecutionsOutput) with field(s):
    ///   - [`notebook_executions(Option<Vec<NotebookExecutionSummary>>)`](crate::output::ListNotebookExecutionsOutput::notebook_executions): <p>A list of notebook executions.</p>
    ///   - [`marker(Option<String>)`](crate::output::ListNotebookExecutionsOutput::marker): <p>A pagination token that a subsequent <code>ListNotebookExecutions</code> can use to determine the next set of results to retrieve.</p>
                            /// - On failure, responds with [`SdkError<ListNotebookExecutionsError>`](crate::error::ListNotebookExecutionsError)
    pub fn list_notebook_executions(&self) -> crate::client::fluent_builders::ListNotebookExecutions {
                                crate::client::fluent_builders::ListNotebookExecutions::new(self.handle.clone())
                            }
}

