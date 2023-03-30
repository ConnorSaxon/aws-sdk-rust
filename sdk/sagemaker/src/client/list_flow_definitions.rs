// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFlowDefinitions`](crate::client::fluent_builders::ListFlowDefinitions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFlowDefinitions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`creation_time_after(DateTime)`](crate::client::fluent_builders::ListFlowDefinitions::creation_time_after) / [`set_creation_time_after(Option<DateTime>)`](crate::client::fluent_builders::ListFlowDefinitions::set_creation_time_after): <p>A filter that returns only flow definitions with a creation time greater than or equal to the specified timestamp.</p>
    ///   - [`creation_time_before(DateTime)`](crate::client::fluent_builders::ListFlowDefinitions::creation_time_before) / [`set_creation_time_before(Option<DateTime>)`](crate::client::fluent_builders::ListFlowDefinitions::set_creation_time_before): <p>A filter that returns only flow definitions that were created before the specified timestamp.</p>
    ///   - [`sort_order(SortOrder)`](crate::client::fluent_builders::ListFlowDefinitions::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::client::fluent_builders::ListFlowDefinitions::set_sort_order): <p>An optional value that specifies whether you want the results sorted in <code>Ascending</code> or <code>Descending</code> order.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFlowDefinitions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFlowDefinitions::set_next_token): <p>A token to resume pagination.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFlowDefinitions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFlowDefinitions::set_max_results): <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> will be provided in the output that you can use to resume pagination.</p>
                            /// - On success, responds with [`ListFlowDefinitionsOutput`](crate::output::ListFlowDefinitionsOutput) with field(s):
    ///   - [`flow_definition_summaries(Option<Vec<FlowDefinitionSummary>>)`](crate::output::ListFlowDefinitionsOutput::flow_definition_summaries): <p>An array of objects describing the flow definitions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListFlowDefinitionsOutput::next_token): <p>A token to resume pagination.</p>
                            /// - On failure, responds with [`SdkError<ListFlowDefinitionsError>`](crate::error::ListFlowDefinitionsError)
    pub fn list_flow_definitions(&self) -> crate::client::fluent_builders::ListFlowDefinitions {
                                crate::client::fluent_builders::ListFlowDefinitions::new(self.handle.clone())
                            }
}

