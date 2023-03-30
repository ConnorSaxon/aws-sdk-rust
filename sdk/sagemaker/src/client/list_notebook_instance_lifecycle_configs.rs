// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListNotebookInstanceLifecycleConfigs`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_next_token): <p>If the result of a <code>ListNotebookInstanceLifecycleConfigs</code> request was truncated, the response includes a <code>NextToken</code>. To get the next set of lifecycle configurations, use the token in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_max_results): <p>The maximum number of lifecycle configurations to return in the response.</p>
    ///   - [`sort_by(NotebookInstanceLifecycleConfigSortKey)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::sort_by) / [`set_sort_by(Option<NotebookInstanceLifecycleConfigSortKey>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_sort_by): <p>Sorts the list of results. The default is <code>CreationTime</code>.</p>
    ///   - [`sort_order(NotebookInstanceLifecycleConfigSortOrder)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::sort_order) / [`set_sort_order(Option<NotebookInstanceLifecycleConfigSortOrder>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_sort_order): <p>The sort order for results.</p>
    ///   - [`name_contains(impl Into<String>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::name_contains) / [`set_name_contains(Option<String>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_name_contains): <p>A string in the lifecycle configuration name. This filter returns only lifecycle configurations whose name contains the specified string.</p>
    ///   - [`creation_time_before(DateTime)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::creation_time_before) / [`set_creation_time_before(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_creation_time_before): <p>A filter that returns only lifecycle configurations that were created before the specified time (timestamp).</p>
    ///   - [`creation_time_after(DateTime)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::creation_time_after) / [`set_creation_time_after(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_creation_time_after): <p>A filter that returns only lifecycle configurations that were created after the specified time (timestamp).</p>
    ///   - [`last_modified_time_before(DateTime)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::last_modified_time_before) / [`set_last_modified_time_before(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_last_modified_time_before): <p>A filter that returns only lifecycle configurations that were modified before the specified time (timestamp).</p>
    ///   - [`last_modified_time_after(DateTime)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::last_modified_time_after) / [`set_last_modified_time_after(Option<DateTime>)`](crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::set_last_modified_time_after): <p>A filter that returns only lifecycle configurations that were modified after the specified time (timestamp).</p>
                            /// - On success, responds with [`ListNotebookInstanceLifecycleConfigsOutput`](crate::output::ListNotebookInstanceLifecycleConfigsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListNotebookInstanceLifecycleConfigsOutput::next_token): <p>If the response is truncated, SageMaker returns this token. To get the next set of lifecycle configurations, use it in the next request. </p>
    ///   - [`notebook_instance_lifecycle_configs(Option<Vec<NotebookInstanceLifecycleConfigSummary>>)`](crate::output::ListNotebookInstanceLifecycleConfigsOutput::notebook_instance_lifecycle_configs): <p>An array of <code>NotebookInstanceLifecycleConfiguration</code> objects, each listing a lifecycle configuration.</p>
                            /// - On failure, responds with [`SdkError<ListNotebookInstanceLifecycleConfigsError>`](crate::error::ListNotebookInstanceLifecycleConfigsError)
    pub fn list_notebook_instance_lifecycle_configs(&self) -> crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs {
                                crate::client::fluent_builders::ListNotebookInstanceLifecycleConfigs::new(self.handle.clone())
                            }
}

