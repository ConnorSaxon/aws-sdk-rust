// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTasks`](crate::client::fluent_builders::ListTasks) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTasks::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListTasks::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListTasks::set_max_results): <p>The maximum number of tasks to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTasks::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTasks::set_next_token): <p>An opaque string that indicates the position at which to begin the next list of tasks.</p>
    ///   - [`filters(Vec<TaskFilter>)`](crate::client::fluent_builders::ListTasks::filters) / [`set_filters(Option<Vec<TaskFilter>>)`](crate::client::fluent_builders::ListTasks::set_filters): <p>You can use API filters to narrow down the list of resources returned by <code>ListTasks</code>. For example, to retrieve all tasks on a specific source location, you can use <code>ListTasks</code> with filter name <code>LocationId</code> and <code>Operator Equals</code> with the ARN for the location.</p>
                            /// - On success, responds with [`ListTasksOutput`](crate::output::ListTasksOutput) with field(s):
    ///   - [`tasks(Option<Vec<TaskListEntry>>)`](crate::output::ListTasksOutput::tasks): <p>A list of all the tasks that are returned.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTasksOutput::next_token): <p>An opaque string that indicates the position at which to begin returning the next list of tasks.</p>
                            /// - On failure, responds with [`SdkError<ListTasksError>`](crate::error::ListTasksError)
    pub fn list_tasks(&self) -> crate::client::fluent_builders::ListTasks {
                                crate::client::fluent_builders::ListTasks::new(self.handle.clone())
                            }
}

