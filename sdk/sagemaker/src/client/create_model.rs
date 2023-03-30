// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateModel`](crate::client::fluent_builders::CreateModel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_name(impl Into<String>)`](crate::client::fluent_builders::CreateModel::model_name) / [`set_model_name(Option<String>)`](crate::client::fluent_builders::CreateModel::set_model_name): <p>The name of the new model.</p>
    ///   - [`primary_container(ContainerDefinition)`](crate::client::fluent_builders::CreateModel::primary_container) / [`set_primary_container(Option<ContainerDefinition>)`](crate::client::fluent_builders::CreateModel::set_primary_container): <p>The location of the primary docker image containing inference code, associated artifacts, and custom environment map that the inference code uses when the model is deployed for predictions. </p>
    ///   - [`containers(Vec<ContainerDefinition>)`](crate::client::fluent_builders::CreateModel::containers) / [`set_containers(Option<Vec<ContainerDefinition>>)`](crate::client::fluent_builders::CreateModel::set_containers): <p>Specifies the containers in the inference pipeline.</p>
    ///   - [`inference_execution_config(InferenceExecutionConfig)`](crate::client::fluent_builders::CreateModel::inference_execution_config) / [`set_inference_execution_config(Option<InferenceExecutionConfig>)`](crate::client::fluent_builders::CreateModel::set_inference_execution_config): <p>Specifies details of how containers in a multi-container endpoint are called.</p>
    ///   - [`execution_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateModel::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::client::fluent_builders::CreateModel::set_execution_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role that SageMaker can assume to access model artifacts and docker image for deployment on ML compute instances or for batch transform jobs. Deploying on ML compute instances is part of model hosting. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker Roles</a>. </p> <note>   <p>To be able to pass this role to SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p>  </note>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateModel::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateModel::set_tags): <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    ///   - [`vpc_config(VpcConfig)`](crate::client::fluent_builders::CreateModel::vpc_config) / [`set_vpc_config(Option<VpcConfig>)`](crate::client::fluent_builders::CreateModel::set_vpc_config): <p>A <code>VpcConfig</code> object that specifies the VPC that you want your model to connect to. Control access to and from your model container by configuring the VPC. <code>VpcConfig</code> is used in hosting services and in batch transform. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-vpc.html">Protect Data in Batch Transform Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    ///   - [`enable_network_isolation(bool)`](crate::client::fluent_builders::CreateModel::enable_network_isolation) / [`set_enable_network_isolation(bool)`](crate::client::fluent_builders::CreateModel::set_enable_network_isolation): <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
                            /// - On success, responds with [`CreateModelOutput`](crate::output::CreateModelOutput) with field(s):
    ///   - [`model_arn(Option<String>)`](crate::output::CreateModelOutput::model_arn): <p>The ARN of the model created in SageMaker.</p>
                            /// - On failure, responds with [`SdkError<CreateModelError>`](crate::error::CreateModelError)
    pub fn create_model(&self) -> crate::client::fluent_builders::CreateModel {
                                crate::client::fluent_builders::CreateModel::new(self.handle.clone())
                            }
}

