// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPipelineState`](crate::client::fluent_builders::GetPipelineState) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetPipelineState::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetPipelineState::set_name): <p>The name of the pipeline about which you want to get information.</p>
                            /// - On success, responds with [`GetPipelineStateOutput`](crate::output::GetPipelineStateOutput) with field(s):
    ///   - [`pipeline_name(Option<String>)`](crate::output::GetPipelineStateOutput::pipeline_name): <p>The name of the pipeline for which you want to get the state.</p>
    ///   - [`pipeline_version(Option<i32>)`](crate::output::GetPipelineStateOutput::pipeline_version): <p>The version number of the pipeline.</p> <note>   <p>A newly created pipeline is always assigned a version number of <code>1</code>.</p>  </note>
    ///   - [`stage_states(Option<Vec<StageState>>)`](crate::output::GetPipelineStateOutput::stage_states): <p>A list of the pipeline stage output information, including stage name, state, most recent run details, whether the stage is disabled, and other data.</p>
    ///   - [`created(Option<DateTime>)`](crate::output::GetPipelineStateOutput::created): <p>The date and time the pipeline was created, in timestamp format.</p>
    ///   - [`updated(Option<DateTime>)`](crate::output::GetPipelineStateOutput::updated): <p>The date and time the pipeline was last updated, in timestamp format.</p>
                            /// - On failure, responds with [`SdkError<GetPipelineStateError>`](crate::error::GetPipelineStateError)
    pub fn get_pipeline_state(&self) -> crate::client::fluent_builders::GetPipelineState {
                                crate::client::fluent_builders::GetPipelineState::new(self.handle.clone())
                            }
}

