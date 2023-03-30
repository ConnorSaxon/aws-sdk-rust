// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDeployment`](crate::client::fluent_builders::GetDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl Into<String>)`](crate::client::fluent_builders::GetDeployment::deployment_id) / [`set_deployment_id(Option<String>)`](crate::client::fluent_builders::GetDeployment::set_deployment_id): <p>The unique identifier for the deployment.</p>
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetDeployment::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetDeployment::set_application_id): <p>The unique identifier of the application.</p>
                            /// - On success, responds with [`GetDeploymentOutput`](crate::output::GetDeploymentOutput) with field(s):
    ///   - [`deployment_id(Option<String>)`](crate::output::GetDeploymentOutput::deployment_id): <p>The unique identifier of the deployment.</p>
    ///   - [`application_id(Option<String>)`](crate::output::GetDeploymentOutput::application_id): <p>The unique identifier of the application.</p>
    ///   - [`environment_id(Option<String>)`](crate::output::GetDeploymentOutput::environment_id): <p>The unique identifier of the runtime environment.</p>
    ///   - [`application_version(Option<i32>)`](crate::output::GetDeploymentOutput::application_version): <p>The application version.</p>
    ///   - [`status(Option<DeploymentLifecycle>)`](crate::output::GetDeploymentOutput::status): <p>The status of the deployment.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::GetDeploymentOutput::creation_time): <p>The timestamp when the deployment was created.</p>
    ///   - [`status_reason(Option<String>)`](crate::output::GetDeploymentOutput::status_reason): <p>The reason for the reported status.</p>
                            /// - On failure, responds with [`SdkError<GetDeploymentError>`](crate::error::GetDeploymentError)
    pub fn get_deployment(&self) -> crate::client::fluent_builders::GetDeployment {
                                crate::client::fluent_builders::GetDeployment::new(self.handle.clone())
                            }
}

