// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopBuildBatch`](crate::client::fluent_builders::StopBuildBatch) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::StopBuildBatch::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::StopBuildBatch::set_id): <p>The identifier of the batch build to stop.</p>
                            /// - On success, responds with [`StopBuildBatchOutput`](crate::output::StopBuildBatchOutput) with field(s):
    ///   - [`build_batch(Option<BuildBatch>)`](crate::output::StopBuildBatchOutput::build_batch): <p>Contains information about a batch build.</p>
                            /// - On failure, responds with [`SdkError<StopBuildBatchError>`](crate::error::StopBuildBatchError)
    pub fn stop_build_batch(&self) -> crate::client::fluent_builders::StopBuildBatch {
                                crate::client::fluent_builders::StopBuildBatch::new(self.handle.clone())
                            }
}

