// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddFlowOutputs`](crate::client::fluent_builders::AddFlowOutputs) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`flow_arn(impl Into<String>)`](crate::client::fluent_builders::AddFlowOutputs::flow_arn) / [`set_flow_arn(Option<String>)`](crate::client::fluent_builders::AddFlowOutputs::set_flow_arn): The flow that you want to add outputs to.
    ///   - [`outputs(Vec<AddOutputRequest>)`](crate::client::fluent_builders::AddFlowOutputs::outputs) / [`set_outputs(Option<Vec<AddOutputRequest>>)`](crate::client::fluent_builders::AddFlowOutputs::set_outputs): A list of outputs that you want to add.
                            /// - On success, responds with [`AddFlowOutputsOutput`](crate::output::AddFlowOutputsOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::output::AddFlowOutputsOutput::flow_arn): The ARN of the flow that these outputs were added to.
    ///   - [`outputs(Option<Vec<Output>>)`](crate::output::AddFlowOutputsOutput::outputs): The details of the newly added outputs.
                            /// - On failure, responds with [`SdkError<AddFlowOutputsError>`](crate::error::AddFlowOutputsError)
    pub fn add_flow_outputs(&self) -> crate::client::fluent_builders::AddFlowOutputs {
                                crate::client::fluent_builders::AddFlowOutputs::new(self.handle.clone())
                            }
}

