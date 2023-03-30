// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopFlow`](crate::client::fluent_builders::StopFlow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`flow_name(impl Into<String>)`](crate::client::fluent_builders::StopFlow::flow_name) / [`set_flow_name(Option<String>)`](crate::client::fluent_builders::StopFlow::set_flow_name): <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
                            /// - On success, responds with [`StopFlowOutput`](crate::output::StopFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::output::StopFlowOutput::flow_arn): <p> The flow's Amazon Resource Name (ARN). </p>
    ///   - [`flow_status(Option<FlowStatus>)`](crate::output::StopFlowOutput::flow_status): <p> Indicates the current status of the flow. </p>
                            /// - On failure, responds with [`SdkError<StopFlowError>`](crate::error::StopFlowError)
    pub fn stop_flow(&self) -> crate::client::fluent_builders::StopFlow {
                                crate::client::fluent_builders::StopFlow::new(self.handle.clone())
                            }
}

