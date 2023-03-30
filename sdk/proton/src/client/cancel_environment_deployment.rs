// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelEnvironmentDeployment`](crate::client::fluent_builders::CancelEnvironmentDeployment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::CancelEnvironmentDeployment::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::CancelEnvironmentDeployment::set_environment_name): <p>The name of the environment with the deployment to cancel.</p>
                            /// - On success, responds with [`CancelEnvironmentDeploymentOutput`](crate::output::CancelEnvironmentDeploymentOutput) with field(s):
    ///   - [`environment(Option<Environment>)`](crate::output::CancelEnvironmentDeploymentOutput::environment): <p>The environment summary data that's returned by Proton.</p>
                            /// - On failure, responds with [`SdkError<CancelEnvironmentDeploymentError>`](crate::error::CancelEnvironmentDeploymentError)
    pub fn cancel_environment_deployment(&self) -> crate::client::fluent_builders::CancelEnvironmentDeployment {
                                crate::client::fluent_builders::CancelEnvironmentDeployment::new(self.handle.clone())
                            }
}

