// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TerminateJobFlows`](crate::client::fluent_builders::TerminateJobFlows) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_flow_ids(Vec<String>)`](crate::client::fluent_builders::TerminateJobFlows::job_flow_ids) / [`set_job_flow_ids(Option<Vec<String>>)`](crate::client::fluent_builders::TerminateJobFlows::set_job_flow_ids): <p>A list of job flows to be shut down.</p>
                            /// - On success, responds with [`TerminateJobFlowsOutput`](crate::output::TerminateJobFlowsOutput)
                            /// - On failure, responds with [`SdkError<TerminateJobFlowsError>`](crate::error::TerminateJobFlowsError)
    pub fn terminate_job_flows(&self) -> crate::client::fluent_builders::TerminateJobFlows {
                                crate::client::fluent_builders::TerminateJobFlows::new(self.handle.clone())
                            }
}

