// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTask`](crate::client::fluent_builders::CreateTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_location_arn(impl Into<String>)`](crate::client::fluent_builders::CreateTask::source_location_arn) / [`set_source_location_arn(Option<String>)`](crate::client::fluent_builders::CreateTask::set_source_location_arn): <p>The Amazon Resource Name (ARN) of the source location for the task.</p>
    ///   - [`destination_location_arn(impl Into<String>)`](crate::client::fluent_builders::CreateTask::destination_location_arn) / [`set_destination_location_arn(Option<String>)`](crate::client::fluent_builders::CreateTask::set_destination_location_arn): <p>The Amazon Resource Name (ARN) of an Amazon Web Services storage resource's location. </p>
    ///   - [`cloud_watch_log_group_arn(impl Into<String>)`](crate::client::fluent_builders::CreateTask::cloud_watch_log_group_arn) / [`set_cloud_watch_log_group_arn(Option<String>)`](crate::client::fluent_builders::CreateTask::set_cloud_watch_log_group_arn): <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to monitor and log events in the task. </p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateTask::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateTask::set_name): <p>The name of a task. This value is a text reference that is used to identify the task in the console. </p>
    ///   - [`options(Options)`](crate::client::fluent_builders::CreateTask::options) / [`set_options(Option<Options>)`](crate::client::fluent_builders::CreateTask::set_options): <p>Specifies the configuration options for a task. Some options include preserving file or object metadata and verifying data integrity.</p>  <p>You can also override these options before starting an individual run of a task (also known as a <i>task execution</i>). For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_StartTaskExecution.html">StartTaskExecution</a>.</p>
    ///   - [`excludes(Vec<FilterRule>)`](crate::client::fluent_builders::CreateTask::excludes) / [`set_excludes(Option<Vec<FilterRule>>)`](crate::client::fluent_builders::CreateTask::set_excludes): <p>Specifies a list of filter rules that exclude specific data during your transfer. For more information and examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Filtering data transferred by DataSync</a>.</p>
    ///   - [`schedule(TaskSchedule)`](crate::client::fluent_builders::CreateTask::schedule) / [`set_schedule(Option<TaskSchedule>)`](crate::client::fluent_builders::CreateTask::set_schedule): <p>Specifies a schedule used to periodically transfer files from a source to a destination location. The schedule should be specified in UTC time. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-scheduling.html">Scheduling your task</a>.</p>
    ///   - [`tags(Vec<TagListEntry>)`](crate::client::fluent_builders::CreateTask::tags) / [`set_tags(Option<Vec<TagListEntry>>)`](crate::client::fluent_builders::CreateTask::set_tags): <p>Specifies the tags that you want to apply to the Amazon Resource Name (ARN) representing the task.</p>  <p> <i>Tags</i> are key-value pairs that help you manage, filter, and search for your DataSync resources.</p>
    ///   - [`includes(Vec<FilterRule>)`](crate::client::fluent_builders::CreateTask::includes) / [`set_includes(Option<Vec<FilterRule>>)`](crate::client::fluent_builders::CreateTask::set_includes): <p>Specifies a list of filter rules that include specific data during your transfer. For more information and examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Filtering data transferred by DataSync</a>.</p>
                            /// - On success, responds with [`CreateTaskOutput`](crate::output::CreateTaskOutput) with field(s):
    ///   - [`task_arn(Option<String>)`](crate::output::CreateTaskOutput::task_arn): <p>The Amazon Resource Name (ARN) of the task.</p>
                            /// - On failure, responds with [`SdkError<CreateTaskError>`](crate::error::CreateTaskError)
    pub fn create_task(&self) -> crate::client::fluent_builders::CreateTask {
                                crate::client::fluent_builders::CreateTask::new(self.handle.clone())
                            }
}

