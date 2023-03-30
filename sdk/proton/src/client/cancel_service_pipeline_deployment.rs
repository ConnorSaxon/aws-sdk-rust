// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelServicePipelineDeployment`](crate::client::fluent_builders::CancelServicePipelineDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`service_name(impl Into<String>)`](crate::client::fluent_builders::CancelServicePipelineDeployment::service_name) / [`set_service_name(Option<String>)`](crate::client::fluent_builders::CancelServicePipelineDeployment::set_service_name): <p>The name of the service with the service pipeline deployment to cancel.</p>
                            /// - On success, responds with [`CancelServicePipelineDeploymentOutput`](crate::output::CancelServicePipelineDeploymentOutput) with field(s):
    ///   - [`pipeline(Option<ServicePipeline>)`](crate::output::CancelServicePipelineDeploymentOutput::pipeline): <p>The service pipeline detail data that's returned by Proton.</p>
                            /// - On failure, responds with [`SdkError<CancelServicePipelineDeploymentError>`](crate::error::CancelServicePipelineDeploymentError)
    pub fn cancel_service_pipeline_deployment(&self) -> crate::client::fluent_builders::CancelServicePipelineDeployment {
                                crate::client::fluent_builders::CancelServicePipelineDeployment::new(self.handle.clone())
                            }
}

