// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePipeline`](crate::client::fluent_builders::DeletePipeline) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pipeline_name(impl Into<String>)`](crate::client::fluent_builders::DeletePipeline::pipeline_name) / [`set_pipeline_name(Option<String>)`](crate::client::fluent_builders::DeletePipeline::set_pipeline_name): <p>The name of the pipeline to delete.</p>
                            /// - On success, responds with [`DeletePipelineOutput`](crate::output::DeletePipelineOutput)
                            /// - On failure, responds with [`SdkError<DeletePipelineError>`](crate::error::DeletePipelineError)
    pub fn delete_pipeline(&self) -> crate::client::fluent_builders::DeletePipeline {
                                crate::client::fluent_builders::DeletePipeline::new(self.handle.clone())
                            }
}

