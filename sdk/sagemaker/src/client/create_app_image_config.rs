// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAppImageConfig`](crate::client::fluent_builders::CreateAppImageConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_image_config_name(impl Into<String>)`](crate::client::fluent_builders::CreateAppImageConfig::app_image_config_name) / [`set_app_image_config_name(Option<String>)`](crate::client::fluent_builders::CreateAppImageConfig::set_app_image_config_name): <p>The name of the AppImageConfig. Must be unique to your account.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAppImageConfig::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAppImageConfig::set_tags): <p>A list of tags to apply to the AppImageConfig.</p>
    ///   - [`kernel_gateway_image_config(KernelGatewayImageConfig)`](crate::client::fluent_builders::CreateAppImageConfig::kernel_gateway_image_config) / [`set_kernel_gateway_image_config(Option<KernelGatewayImageConfig>)`](crate::client::fluent_builders::CreateAppImageConfig::set_kernel_gateway_image_config): <p>The KernelGatewayImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel will be shown to users before the image starts. Once the image runs, all kernels are visible in JupyterLab.</p>
                            /// - On success, responds with [`CreateAppImageConfigOutput`](crate::output::CreateAppImageConfigOutput) with field(s):
    ///   - [`app_image_config_arn(Option<String>)`](crate::output::CreateAppImageConfigOutput::app_image_config_arn): <p>The Amazon Resource Name (ARN) of the AppImageConfig.</p>
                            /// - On failure, responds with [`SdkError<CreateAppImageConfigError>`](crate::error::CreateAppImageConfigError)
    pub fn create_app_image_config(&self) -> crate::client::fluent_builders::CreateAppImageConfig {
                                crate::client::fluent_builders::CreateAppImageConfig::new(self.handle.clone())
                            }
}

