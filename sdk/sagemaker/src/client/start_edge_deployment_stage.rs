// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartEdgeDeploymentStage`](crate::client::fluent_builders::StartEdgeDeploymentStage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`edge_deployment_plan_name(impl Into<String>)`](crate::client::fluent_builders::StartEdgeDeploymentStage::edge_deployment_plan_name) / [`set_edge_deployment_plan_name(Option<String>)`](crate::client::fluent_builders::StartEdgeDeploymentStage::set_edge_deployment_plan_name): <p>The name of the edge deployment plan to start.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::StartEdgeDeploymentStage::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::StartEdgeDeploymentStage::set_stage_name): <p>The name of the stage to start.</p>
                            /// - On success, responds with [`StartEdgeDeploymentStageOutput`](crate::output::StartEdgeDeploymentStageOutput)
                            /// - On failure, responds with [`SdkError<StartEdgeDeploymentStageError>`](crate::error::StartEdgeDeploymentStageError)
    pub fn start_edge_deployment_stage(&self) -> crate::client::fluent_builders::StartEdgeDeploymentStage {
                                crate::client::fluent_builders::StartEdgeDeploymentStage::new(self.handle.clone())
                            }
}

