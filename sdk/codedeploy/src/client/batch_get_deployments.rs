// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetDeployments`](crate::client::fluent_builders::BatchGetDeployments) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_ids(Vec<String>)`](crate::client::fluent_builders::BatchGetDeployments::deployment_ids) / [`set_deployment_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetDeployments::set_deployment_ids): <p> A list of deployment IDs, separated by spaces. The maximum number of deployment IDs you can specify is 25.</p>
                            /// - On success, responds with [`BatchGetDeploymentsOutput`](crate::output::BatchGetDeploymentsOutput) with field(s):
    ///   - [`deployments_info(Option<Vec<DeploymentInfo>>)`](crate::output::BatchGetDeploymentsOutput::deployments_info): <p> Information about the deployments. </p>
                            /// - On failure, responds with [`SdkError<BatchGetDeploymentsError>`](crate::error::BatchGetDeploymentsError)
    pub fn batch_get_deployments(&self) -> crate::client::fluent_builders::BatchGetDeployments {
                                crate::client::fluent_builders::BatchGetDeployments::new(self.handle.clone())
                            }
}

