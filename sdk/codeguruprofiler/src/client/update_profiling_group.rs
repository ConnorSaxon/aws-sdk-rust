// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateProfilingGroup`](crate::client::fluent_builders::UpdateProfilingGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`profiling_group_name(impl Into<String>)`](crate::client::fluent_builders::UpdateProfilingGroup::profiling_group_name) / [`set_profiling_group_name(Option<String>)`](crate::client::fluent_builders::UpdateProfilingGroup::set_profiling_group_name): <p>The name of the profiling group to update.</p>
    ///   - [`agent_orchestration_config(AgentOrchestrationConfig)`](crate::client::fluent_builders::UpdateProfilingGroup::agent_orchestration_config) / [`set_agent_orchestration_config(Option<AgentOrchestrationConfig>)`](crate::client::fluent_builders::UpdateProfilingGroup::set_agent_orchestration_config): <p> Specifies whether profiling is enabled or disabled for a profiling group. </p>
                            /// - On success, responds with [`UpdateProfilingGroupOutput`](crate::output::UpdateProfilingGroupOutput) with field(s):
    ///   - [`profiling_group(Option<ProfilingGroupDescription>)`](crate::output::UpdateProfilingGroupOutput::profiling_group): <p> A <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> that contains information about the returned updated profiling group. </p>
                            /// - On failure, responds with [`SdkError<UpdateProfilingGroupError>`](crate::error::UpdateProfilingGroupError)
    pub fn update_profiling_group(&self) -> crate::client::fluent_builders::UpdateProfilingGroup {
                                crate::client::fluent_builders::UpdateProfilingGroup::new(self.handle.clone())
                            }
}

