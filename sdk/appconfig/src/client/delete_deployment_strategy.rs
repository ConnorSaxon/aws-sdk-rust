// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDeploymentStrategy`](crate::client::fluent_builders::DeleteDeploymentStrategy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_strategy_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDeploymentStrategy::deployment_strategy_id) / [`set_deployment_strategy_id(Option<String>)`](crate::client::fluent_builders::DeleteDeploymentStrategy::set_deployment_strategy_id): <p>The ID of the deployment strategy you want to delete.</p>
                            /// - On success, responds with [`DeleteDeploymentStrategyOutput`](crate::output::DeleteDeploymentStrategyOutput)
                            /// - On failure, responds with [`SdkError<DeleteDeploymentStrategyError>`](crate::error::DeleteDeploymentStrategyError)
    pub fn delete_deployment_strategy(&self) -> crate::client::fluent_builders::DeleteDeploymentStrategy {
                                crate::client::fluent_builders::DeleteDeploymentStrategy::new(self.handle.clone())
                            }
}

