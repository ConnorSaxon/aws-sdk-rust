// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMediaCapturePipeline`](crate::client::fluent_builders::GetMediaCapturePipeline) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`media_pipeline_id(impl Into<String>)`](crate::client::fluent_builders::GetMediaCapturePipeline::media_pipeline_id) / [`set_media_pipeline_id(Option<String>)`](crate::client::fluent_builders::GetMediaCapturePipeline::set_media_pipeline_id): <p>The ID of the pipeline that you want to get.</p>
                            /// - On success, responds with [`GetMediaCapturePipelineOutput`](crate::output::GetMediaCapturePipelineOutput) with field(s):
    ///   - [`media_capture_pipeline(Option<MediaCapturePipeline>)`](crate::output::GetMediaCapturePipelineOutput::media_capture_pipeline): <p>The media capture pipeline object.</p>
                            /// - On failure, responds with [`SdkError<GetMediaCapturePipelineError>`](crate::error::GetMediaCapturePipelineError)
    pub fn get_media_capture_pipeline(&self) -> crate::client::fluent_builders::GetMediaCapturePipeline {
                                crate::client::fluent_builders::GetMediaCapturePipeline::new(self.handle.clone())
                            }
}

