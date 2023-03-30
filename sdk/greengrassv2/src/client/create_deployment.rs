// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDeployment`](crate::client::fluent_builders::CreateDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`target_arn(impl Into<String>)`](crate::client::fluent_builders::CreateDeployment::target_arn) / [`set_target_arn(Option<String>)`](crate::client::fluent_builders::CreateDeployment::set_target_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group. When creating a subdeployment, the targetARN can only be a thing group.</p>
    ///   - [`deployment_name(impl Into<String>)`](crate::client::fluent_builders::CreateDeployment::deployment_name) / [`set_deployment_name(Option<String>)`](crate::client::fluent_builders::CreateDeployment::set_deployment_name): <p>The name of the deployment.</p>
    ///   - [`components(HashMap<String, ComponentDeploymentSpecification>)`](crate::client::fluent_builders::CreateDeployment::components) / [`set_components(Option<HashMap<String, ComponentDeploymentSpecification>>)`](crate::client::fluent_builders::CreateDeployment::set_components): <p>The components to deploy. This is a dictionary, where each key is the name of a component, and each key's value is the version and configuration to deploy for that component.</p>
    ///   - [`iot_job_configuration(DeploymentIoTJobConfiguration)`](crate::client::fluent_builders::CreateDeployment::iot_job_configuration) / [`set_iot_job_configuration(Option<DeploymentIoTJobConfiguration>)`](crate::client::fluent_builders::CreateDeployment::set_iot_job_configuration): <p>The job configuration for the deployment configuration. The job configuration specifies the rollout, timeout, and stop configurations for the deployment configuration.</p>
    ///   - [`deployment_policies(DeploymentPolicies)`](crate::client::fluent_builders::CreateDeployment::deployment_policies) / [`set_deployment_policies(Option<DeploymentPolicies>)`](crate::client::fluent_builders::CreateDeployment::set_deployment_policies): <p>The deployment policies for the deployment. These policies define how the deployment updates components and handles failure.</p>
    ///   - [`parent_target_arn(impl Into<String>)`](crate::client::fluent_builders::CreateDeployment::parent_target_arn) / [`set_parent_target_arn(Option<String>)`](crate::client::fluent_builders::CreateDeployment::set_parent_target_arn): <p>The parent deployment's target <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> within a subdeployment.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateDeployment::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateDeployment::set_tags): <p>A list of key-value pairs that contain metadata for the resource. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateDeployment::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateDeployment::set_client_token): <p>A unique, case-sensitive identifier that you can provide to ensure that the request is idempotent. Idempotency means that the request is successfully processed only once, even if you send the request multiple times. When a request succeeds, and you specify the same client token for subsequent successful requests, the IoT Greengrass V2 service returns the successful response that it caches from the previous request. IoT Greengrass V2 caches successful responses for idempotent requests for up to 8 hours.</p>
                            /// - On success, responds with [`CreateDeploymentOutput`](crate::output::CreateDeploymentOutput) with field(s):
    ///   - [`deployment_id(Option<String>)`](crate::output::CreateDeploymentOutput::deployment_id): <p>The ID of the deployment.</p>
    ///   - [`iot_job_id(Option<String>)`](crate::output::CreateDeploymentOutput::iot_job_id): <p>The ID of the IoT job that applies the deployment to target devices.</p>
    ///   - [`iot_job_arn(Option<String>)`](crate::output::CreateDeploymentOutput::iot_job_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IoT job that applies the deployment to target devices.</p>
                            /// - On failure, responds with [`SdkError<CreateDeploymentError>`](crate::error::CreateDeploymentError)
    pub fn create_deployment(&self) -> crate::client::fluent_builders::CreateDeployment {
                                crate::client::fluent_builders::CreateDeployment::new(self.handle.clone())
                            }
}

