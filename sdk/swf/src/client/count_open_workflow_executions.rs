// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CountOpenWorkflowExecutions`](crate::client::fluent_builders::CountOpenWorkflowExecutions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::set_domain): <p>The name of the domain containing the workflow executions to count.</p>
    ///   - [`start_time_filter(ExecutionTimeFilter)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::start_time_filter) / [`set_start_time_filter(Option<ExecutionTimeFilter>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::set_start_time_filter): <p>Specifies the start time criteria that workflow executions must meet in order to be counted.</p>
    ///   - [`type_filter(WorkflowTypeFilter)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::type_filter) / [`set_type_filter(Option<WorkflowTypeFilter>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::set_type_filter): <p>Specifies the type of the workflow executions to be counted.</p> <note>   <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>  </note>
    ///   - [`tag_filter(TagFilter)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::tag_filter) / [`set_tag_filter(Option<TagFilter>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::set_tag_filter): <p>If specified, only executions that have a tag that matches the filter are counted.</p> <note>   <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>  </note>
    ///   - [`execution_filter(WorkflowExecutionFilter)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::execution_filter) / [`set_execution_filter(Option<WorkflowExecutionFilter>)`](crate::client::fluent_builders::CountOpenWorkflowExecutions::set_execution_filter): <p>If specified, only workflow executions matching the <code>WorkflowId</code> in the filter are counted.</p> <note>   <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p>  </note>
                            /// - On success, responds with [`CountOpenWorkflowExecutionsOutput`](crate::output::CountOpenWorkflowExecutionsOutput) with field(s):
    ///   - [`count(i32)`](crate::output::CountOpenWorkflowExecutionsOutput::count): <p>The number of workflow executions.</p>
    ///   - [`truncated(bool)`](crate::output::CountOpenWorkflowExecutionsOutput::truncated): <p>If set to true, indicates that the actual count was more than the maximum supported by this API and the count returned is the truncated value.</p>
                            /// - On failure, responds with [`SdkError<CountOpenWorkflowExecutionsError>`](crate::error::CountOpenWorkflowExecutionsError)
    pub fn count_open_workflow_executions(&self) -> crate::client::fluent_builders::CountOpenWorkflowExecutions {
                                crate::client::fluent_builders::CountOpenWorkflowExecutions::new(self.handle.clone())
                            }
}

