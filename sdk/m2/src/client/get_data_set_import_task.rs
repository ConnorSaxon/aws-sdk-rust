// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDataSetImportTask`](crate::client::fluent_builders::GetDataSetImportTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetDataSetImportTask::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetDataSetImportTask::set_application_id): <p>The application identifier.</p>
    ///   - [`task_id(impl Into<String>)`](crate::client::fluent_builders::GetDataSetImportTask::task_id) / [`set_task_id(Option<String>)`](crate::client::fluent_builders::GetDataSetImportTask::set_task_id): <p>The task identifier returned by the <code>CreateDataSetImportTask</code> operation. </p>
                            /// - On success, responds with [`GetDataSetImportTaskOutput`](crate::output::GetDataSetImportTaskOutput) with field(s):
    ///   - [`task_id(Option<String>)`](crate::output::GetDataSetImportTaskOutput::task_id): <p>The task identifier.</p>
    ///   - [`status(Option<DataSetTaskLifecycle>)`](crate::output::GetDataSetImportTaskOutput::status): <p>The status of the task.</p>
    ///   - [`summary(Option<DataSetImportSummary>)`](crate::output::GetDataSetImportTaskOutput::summary): <p>A summary of the status of the task.</p>
                            /// - On failure, responds with [`SdkError<GetDataSetImportTaskError>`](crate::error::GetDataSetImportTaskError)
    pub fn get_data_set_import_task(&self) -> crate::client::fluent_builders::GetDataSetImportTask {
                                crate::client::fluent_builders::GetDataSetImportTask::new(self.handle.clone())
                            }
}

