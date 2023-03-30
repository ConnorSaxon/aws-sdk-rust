// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeactivatePipeline`](crate::client::fluent_builders::DeactivatePipeline) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pipeline_id(impl Into<String>)`](crate::client::fluent_builders::DeactivatePipeline::pipeline_id) / [`set_pipeline_id(Option<String>)`](crate::client::fluent_builders::DeactivatePipeline::set_pipeline_id): <p>The ID of the pipeline.</p>
    ///   - [`cancel_active(bool)`](crate::client::fluent_builders::DeactivatePipeline::cancel_active) / [`set_cancel_active(Option<bool>)`](crate::client::fluent_builders::DeactivatePipeline::set_cancel_active): <p>Indicates whether to cancel any running objects. The default is true, which sets the state of any running objects to <code>CANCELED</code>. If this value is false, the pipeline is deactivated after all running objects finish.</p>
                            /// - On success, responds with [`DeactivatePipelineOutput`](crate::output::DeactivatePipelineOutput)
                            /// - On failure, responds with [`SdkError<DeactivatePipelineError>`](crate::error::DeactivatePipelineError)
    pub fn deactivate_pipeline(&self) -> crate::client::fluent_builders::DeactivatePipeline {
                                crate::client::fluent_builders::DeactivatePipeline::new(self.handle.clone())
                            }
}

