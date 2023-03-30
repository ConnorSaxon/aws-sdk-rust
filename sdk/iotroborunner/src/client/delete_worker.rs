// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteWorker`](crate::client::fluent_builders::DeleteWorker) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteWorker::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteWorker::set_id): Full ARN of the worker.
                            /// - On success, responds with [`DeleteWorkerOutput`](crate::output::DeleteWorkerOutput)
                            /// - On failure, responds with [`SdkError<DeleteWorkerError>`](crate::error::DeleteWorkerError)
    pub fn delete_worker(&self) -> crate::client::fluent_builders::DeleteWorker {
                                crate::client::fluent_builders::DeleteWorker::new(self.handle.clone())
                            }
}

