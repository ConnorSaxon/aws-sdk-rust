// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMLTaskRun`](crate::client::fluent_builders::GetMLTaskRun) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transform_id(impl Into<String>)`](crate::client::fluent_builders::GetMLTaskRun::transform_id) / [`set_transform_id(Option<String>)`](crate::client::fluent_builders::GetMLTaskRun::set_transform_id): <p>The unique identifier of the machine learning transform.</p>
    ///   - [`task_run_id(impl Into<String>)`](crate::client::fluent_builders::GetMLTaskRun::task_run_id) / [`set_task_run_id(Option<String>)`](crate::client::fluent_builders::GetMLTaskRun::set_task_run_id): <p>The unique identifier of the task run.</p>
                            /// - On success, responds with [`GetMlTaskRunOutput`](crate::output::GetMlTaskRunOutput) with field(s):
    ///   - [`transform_id(Option<String>)`](crate::output::GetMlTaskRunOutput::transform_id): <p>The unique identifier of the task run.</p>
    ///   - [`task_run_id(Option<String>)`](crate::output::GetMlTaskRunOutput::task_run_id): <p>The unique run identifier associated with this run.</p>
    ///   - [`status(Option<TaskStatusType>)`](crate::output::GetMlTaskRunOutput::status): <p>The status for this task run.</p>
    ///   - [`log_group_name(Option<String>)`](crate::output::GetMlTaskRunOutput::log_group_name): <p>The names of the log groups that are associated with the task run.</p>
    ///   - [`properties(Option<TaskRunProperties>)`](crate::output::GetMlTaskRunOutput::properties): <p>The list of properties that are associated with the task run.</p>
    ///   - [`error_string(Option<String>)`](crate::output::GetMlTaskRunOutput::error_string): <p>The error strings that are associated with the task run.</p>
    ///   - [`started_on(Option<DateTime>)`](crate::output::GetMlTaskRunOutput::started_on): <p>The date and time when this task run started.</p>
    ///   - [`last_modified_on(Option<DateTime>)`](crate::output::GetMlTaskRunOutput::last_modified_on): <p>The date and time when this task run was last modified.</p>
    ///   - [`completed_on(Option<DateTime>)`](crate::output::GetMlTaskRunOutput::completed_on): <p>The date and time when this task run was completed.</p>
    ///   - [`execution_time(i32)`](crate::output::GetMlTaskRunOutput::execution_time): <p>The amount of time (in seconds) that the task run consumed resources.</p>
                            /// - On failure, responds with [`SdkError<GetMLTaskRunError>`](crate::error::GetMLTaskRunError)
    pub fn get_ml_task_run(&self) -> crate::client::fluent_builders::GetMLTaskRun {
                                crate::client::fluent_builders::GetMLTaskRun::new(self.handle.clone())
                            }
}

