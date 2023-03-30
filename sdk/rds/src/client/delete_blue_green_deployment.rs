// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBlueGreenDeployment`](crate::client::fluent_builders::DeleteBlueGreenDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`blue_green_deployment_identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteBlueGreenDeployment::blue_green_deployment_identifier) / [`set_blue_green_deployment_identifier(Option<String>)`](crate::client::fluent_builders::DeleteBlueGreenDeployment::set_blue_green_deployment_identifier): <p>The blue/green deployment identifier of the deployment to be deleted. This parameter isn't case-sensitive.</p>  <p>Constraints: </p>  <ul>   <li> <p>Must match an existing blue/green deployment identifier.</p> </li>  </ul>
    ///   - [`delete_target(bool)`](crate::client::fluent_builders::DeleteBlueGreenDeployment::delete_target) / [`set_delete_target(Option<bool>)`](crate::client::fluent_builders::DeleteBlueGreenDeployment::set_delete_target): <p>A value that indicates whether to delete the resources in the green environment.</p>
                            /// - On success, responds with [`DeleteBlueGreenDeploymentOutput`](crate::output::DeleteBlueGreenDeploymentOutput) with field(s):
    ///   - [`blue_green_deployment(Option<BlueGreenDeployment>)`](crate::output::DeleteBlueGreenDeploymentOutput::blue_green_deployment): <p>Contains the details about a blue/green deployment.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/blue-green-deployments.html">Using Amazon RDS Blue/Green Deployments for database updates</a> in the <i>Amazon RDS User Guide</i> and <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/blue-green-deployments.html"> Using Amazon RDS Blue/Green Deployments for database updates</a> in the <i>Amazon Aurora User Guide</i>.</p>
                            /// - On failure, responds with [`SdkError<DeleteBlueGreenDeploymentError>`](crate::error::DeleteBlueGreenDeploymentError)
    pub fn delete_blue_green_deployment(&self) -> crate::client::fluent_builders::DeleteBlueGreenDeployment {
                                crate::client::fluent_builders::DeleteBlueGreenDeployment::new(self.handle.clone())
                            }
}

