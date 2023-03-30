// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateEndpointConfig`](crate::client::fluent_builders::CreateEndpointConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`endpoint_config_name(impl Into<String>)`](crate::client::fluent_builders::CreateEndpointConfig::endpoint_config_name) / [`set_endpoint_config_name(Option<String>)`](crate::client::fluent_builders::CreateEndpointConfig::set_endpoint_config_name): <p>The name of the endpoint configuration. You specify this name in a <code>CreateEndpoint</code> request. </p>
    ///   - [`production_variants(Vec<ProductionVariant>)`](crate::client::fluent_builders::CreateEndpointConfig::production_variants) / [`set_production_variants(Option<Vec<ProductionVariant>>)`](crate::client::fluent_builders::CreateEndpointConfig::set_production_variants): <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    ///   - [`data_capture_config(DataCaptureConfig)`](crate::client::fluent_builders::CreateEndpointConfig::data_capture_config) / [`set_data_capture_config(Option<DataCaptureConfig>)`](crate::client::fluent_builders::CreateEndpointConfig::set_data_capture_config): <p>Configuration to control how SageMaker captures inference data.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateEndpointConfig::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateEndpointConfig::set_tags): <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateEndpointConfig::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateEndpointConfig::set_kms_key_id): <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>  <p>The KmsKeyId can be any of the following formats: </p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li>  </ul>  <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note>   <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p>   <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p>   <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p>  </note>
    ///   - [`async_inference_config(AsyncInferenceConfig)`](crate::client::fluent_builders::CreateEndpointConfig::async_inference_config) / [`set_async_inference_config(Option<AsyncInferenceConfig>)`](crate::client::fluent_builders::CreateEndpointConfig::set_async_inference_config): <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p>
    ///   - [`explainer_config(ExplainerConfig)`](crate::client::fluent_builders::CreateEndpointConfig::explainer_config) / [`set_explainer_config(Option<ExplainerConfig>)`](crate::client::fluent_builders::CreateEndpointConfig::set_explainer_config): <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p>
    ///   - [`shadow_production_variants(Vec<ProductionVariant>)`](crate::client::fluent_builders::CreateEndpointConfig::shadow_production_variants) / [`set_shadow_production_variants(Option<Vec<ProductionVariant>>)`](crate::client::fluent_builders::CreateEndpointConfig::set_shadow_production_variants): <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p>
                            /// - On success, responds with [`CreateEndpointConfigOutput`](crate::output::CreateEndpointConfigOutput) with field(s):
    ///   - [`endpoint_config_arn(Option<String>)`](crate::output::CreateEndpointConfigOutput::endpoint_config_arn): <p>The Amazon Resource Name (ARN) of the endpoint configuration. </p>
                            /// - On failure, responds with [`SdkError<CreateEndpointConfigError>`](crate::error::CreateEndpointConfigError)
    pub fn create_endpoint_config(&self) -> crate::client::fluent_builders::CreateEndpointConfig {
                                crate::client::fluent_builders::CreateEndpointConfig::new(self.handle.clone())
                            }
}

