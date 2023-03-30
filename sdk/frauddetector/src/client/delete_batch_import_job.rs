// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBatchImportJob`](crate::client::fluent_builders::DeleteBatchImportJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::DeleteBatchImportJob::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::DeleteBatchImportJob::set_job_id): <p>The ID of the batch import job to delete. </p>
                            /// - On success, responds with [`DeleteBatchImportJobOutput`](crate::output::DeleteBatchImportJobOutput)
                            /// - On failure, responds with [`SdkError<DeleteBatchImportJobError>`](crate::error::DeleteBatchImportJobError)
    pub fn delete_batch_import_job(&self) -> crate::client::fluent_builders::DeleteBatchImportJob {
                                crate::client::fluent_builders::DeleteBatchImportJob::new(self.handle.clone())
                            }
}

