// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreatePipe`](crate::client::fluent_builders::CreatePipe) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_name): <p>The name of the pipe.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_description): <p>A description of the pipe.</p>
    ///   - [`desired_state(RequestedPipeState)`](crate::client::fluent_builders::CreatePipe::desired_state) / [`set_desired_state(Option<RequestedPipeState>)`](crate::client::fluent_builders::CreatePipe::set_desired_state): <p>The state the pipe should be in.</p>
    ///   - [`source(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::source) / [`set_source(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_source): <p>The ARN of the source resource.</p>
    ///   - [`source_parameters(PipeSourceParameters)`](crate::client::fluent_builders::CreatePipe::source_parameters) / [`set_source_parameters(Option<PipeSourceParameters>)`](crate::client::fluent_builders::CreatePipe::set_source_parameters): <p>The parameters required to set up a source for your pipe.</p>
    ///   - [`enrichment(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::enrichment) / [`set_enrichment(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_enrichment): <p>The ARN of the enrichment resource.</p>
    ///   - [`enrichment_parameters(PipeEnrichmentParameters)`](crate::client::fluent_builders::CreatePipe::enrichment_parameters) / [`set_enrichment_parameters(Option<PipeEnrichmentParameters>)`](crate::client::fluent_builders::CreatePipe::set_enrichment_parameters): <p>The parameters required to set up enrichment on your pipe.</p>
    ///   - [`target(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::target) / [`set_target(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_target): <p>The ARN of the target resource.</p>
    ///   - [`target_parameters(PipeTargetParameters)`](crate::client::fluent_builders::CreatePipe::target_parameters) / [`set_target_parameters(Option<PipeTargetParameters>)`](crate::client::fluent_builders::CreatePipe::set_target_parameters): <p>The parameters required to set up a target for your pipe.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_role_arn): <p>The ARN of the role that allows the pipe to send data to the target.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreatePipe::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreatePipe::set_tags): <p>The list of key-value pairs to associate with the pipe.</p>
                            /// - On success, responds with [`CreatePipeOutput`](crate::output::CreatePipeOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreatePipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::CreatePipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeState>)`](crate::output::CreatePipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::CreatePipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::CreatePipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::CreatePipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                            /// - On failure, responds with [`SdkError<CreatePipeError>`](crate::error::CreatePipeError)
    pub fn create_pipe(&self) -> crate::client::fluent_builders::CreatePipe {
                                crate::client::fluent_builders::CreatePipe::new(self.handle.clone())
                            }
}

