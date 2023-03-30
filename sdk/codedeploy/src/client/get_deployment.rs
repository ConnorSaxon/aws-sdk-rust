// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDeployment`](crate::client::fluent_builders::GetDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl Into<String>)`](crate::client::fluent_builders::GetDeployment::deployment_id) / [`set_deployment_id(Option<String>)`](crate::client::fluent_builders::GetDeployment::set_deployment_id): <p> The unique ID of a deployment associated with the IAM user or Amazon Web Services account. </p>
                            /// - On success, responds with [`GetDeploymentOutput`](crate::output::GetDeploymentOutput) with field(s):
    ///   - [`deployment_info(Option<DeploymentInfo>)`](crate::output::GetDeploymentOutput::deployment_info): <p>Information about the deployment.</p>
                            /// - On failure, responds with [`SdkError<GetDeploymentError>`](crate::error::GetDeploymentError)
    pub fn get_deployment(&self) -> crate::client::fluent_builders::GetDeployment {
                                crate::client::fluent_builders::GetDeployment::new(self.handle.clone())
                            }
}

