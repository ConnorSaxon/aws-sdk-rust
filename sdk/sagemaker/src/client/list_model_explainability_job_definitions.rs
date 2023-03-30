// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListModelExplainabilityJobDefinitions`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`endpoint_name(impl Into<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::endpoint_name) / [`set_endpoint_name(Option<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_endpoint_name): <p>Name of the endpoint to monitor for model explainability.</p>
    ///   - [`sort_by(MonitoringJobDefinitionSortKey)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::sort_by) / [`set_sort_by(Option<MonitoringJobDefinitionSortKey>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_sort_by): <p>Whether to sort results by the <code>Name</code> or <code>CreationTime</code> field. The default is <code>CreationTime</code>.</p>
    ///   - [`sort_order(SortOrder)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_sort_order): <p>Whether to sort the results in <code>Ascending</code> or <code>Descending</code> order. The default is <code>Descending</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_next_token): <p>The token returned if the response is truncated. To retrieve the next set of job executions, use it in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_max_results): <p>The maximum number of jobs to return in the response. The default value is 10.</p>
    ///   - [`name_contains(impl Into<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::name_contains) / [`set_name_contains(Option<String>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_name_contains): <p>Filter for model explainability jobs whose name contains a specified string.</p>
    ///   - [`creation_time_before(DateTime)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::creation_time_before) / [`set_creation_time_before(Option<DateTime>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_creation_time_before): <p>A filter that returns only model explainability jobs created before a specified time.</p>
    ///   - [`creation_time_after(DateTime)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::creation_time_after) / [`set_creation_time_after(Option<DateTime>)`](crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::set_creation_time_after): <p>A filter that returns only model explainability jobs created after a specified time.</p>
                            /// - On success, responds with [`ListModelExplainabilityJobDefinitionsOutput`](crate::output::ListModelExplainabilityJobDefinitionsOutput) with field(s):
    ///   - [`job_definition_summaries(Option<Vec<MonitoringJobDefinitionSummary>>)`](crate::output::ListModelExplainabilityJobDefinitionsOutput::job_definition_summaries): <p>A JSON array in which each element is a summary for a explainability bias jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListModelExplainabilityJobDefinitionsOutput::next_token): <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of jobs, use it in the subsequent request.</p>
                            /// - On failure, responds with [`SdkError<ListModelExplainabilityJobDefinitionsError>`](crate::error::ListModelExplainabilityJobDefinitionsError)
    pub fn list_model_explainability_job_definitions(&self) -> crate::client::fluent_builders::ListModelExplainabilityJobDefinitions {
                                crate::client::fluent_builders::ListModelExplainabilityJobDefinitions::new(self.handle.clone())
                            }
}

