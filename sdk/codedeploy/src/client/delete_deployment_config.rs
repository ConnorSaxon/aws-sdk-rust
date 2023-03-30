// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDeploymentConfig`](crate::client::fluent_builders::DeleteDeploymentConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_config_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDeploymentConfig::deployment_config_name) / [`set_deployment_config_name(Option<String>)`](crate::client::fluent_builders::DeleteDeploymentConfig::set_deployment_config_name): <p>The name of a deployment configuration associated with the IAM user or Amazon Web Services account.</p>
                            /// - On success, responds with [`DeleteDeploymentConfigOutput`](crate::output::DeleteDeploymentConfigOutput)
                            /// - On failure, responds with [`SdkError<DeleteDeploymentConfigError>`](crate::error::DeleteDeploymentConfigError)
    pub fn delete_deployment_config(&self) -> crate::client::fluent_builders::DeleteDeploymentConfig {
                                crate::client::fluent_builders::DeleteDeploymentConfig::new(self.handle.clone())
                            }
}

