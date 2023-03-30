// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetDeploymentTargets`](crate::client::fluent_builders::BatchGetDeploymentTargets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl Into<String>)`](crate::client::fluent_builders::BatchGetDeploymentTargets::deployment_id) / [`set_deployment_id(Option<String>)`](crate::client::fluent_builders::BatchGetDeploymentTargets::set_deployment_id): <p> The unique ID of a deployment. </p>
    ///   - [`target_ids(Vec<String>)`](crate::client::fluent_builders::BatchGetDeploymentTargets::target_ids) / [`set_target_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetDeploymentTargets::set_target_ids): <p> The unique IDs of the deployment targets. The compute platform of the deployment determines the type of the targets and their formats. The maximum number of deployment target IDs you can specify is 25.</p>  <ul>   <li> <p> For deployments that use the EC2/On-premises compute platform, the target IDs are Amazon EC2 or on-premises instances IDs, and their target type is <code>instanceTarget</code>. </p> </li>   <li> <p> For deployments that use the Lambda compute platform, the target IDs are the names of Lambda functions, and their target type is <code>instanceTarget</code>. </p> </li>   <li> <p> For deployments that use the Amazon ECS compute platform, the target IDs are pairs of Amazon ECS clusters and services specified using the format <code>     <clustername>      :      <servicename></servicename>     </clustername></code>. Their target type is <code>ecsTarget</code>. </p> </li>   <li> <p> For deployments that are deployed with CloudFormation, the target IDs are CloudFormation stack IDs. Their target type is <code>cloudFormationTarget</code>. </p> </li>  </ul>
                            /// - On success, responds with [`BatchGetDeploymentTargetsOutput`](crate::output::BatchGetDeploymentTargetsOutput) with field(s):
    ///   - [`deployment_targets(Option<Vec<DeploymentTarget>>)`](crate::output::BatchGetDeploymentTargetsOutput::deployment_targets): <p> A list of target objects for a deployment. Each target object contains details about the target, such as its status and lifecycle events. The type of the target objects depends on the deployment' compute platform. </p>  <ul>   <li> <p> <b>EC2/On-premises</b>: Each target object is an Amazon EC2 or on-premises instance. </p> </li>   <li> <p> <b>Lambda</b>: The target object is a specific version of an Lambda function. </p> </li>   <li> <p> <b>Amazon ECS</b>: The target object is an Amazon ECS service. </p> </li>   <li> <p> <b>CloudFormation</b>: The target object is an CloudFormation blue/green deployment. </p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<BatchGetDeploymentTargetsError>`](crate::error::BatchGetDeploymentTargetsError)
    pub fn batch_get_deployment_targets(&self) -> crate::client::fluent_builders::BatchGetDeploymentTargets {
                                crate::client::fluent_builders::BatchGetDeploymentTargets::new(self.handle.clone())
                            }
}

