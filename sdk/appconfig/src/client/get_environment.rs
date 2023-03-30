// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEnvironment`](crate::client::fluent_builders::GetEnvironment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetEnvironment::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetEnvironment::set_application_id): <p>The ID of the application that includes the environment you want to get.</p>
    ///   - [`environment_id(impl Into<String>)`](crate::client::fluent_builders::GetEnvironment::environment_id) / [`set_environment_id(Option<String>)`](crate::client::fluent_builders::GetEnvironment::set_environment_id): <p>The ID of the environment that you want to get.</p>
                            /// - On success, responds with [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::output::GetEnvironmentOutput::application_id): <p>The application ID.</p>
    ///   - [`id(Option<String>)`](crate::output::GetEnvironmentOutput::id): <p>The environment ID.</p>
    ///   - [`name(Option<String>)`](crate::output::GetEnvironmentOutput::name): <p>The name of the environment.</p>
    ///   - [`description(Option<String>)`](crate::output::GetEnvironmentOutput::description): <p>The description of the environment.</p>
    ///   - [`state(Option<EnvironmentState>)`](crate::output::GetEnvironmentOutput::state): <p>The state of the environment. An environment can be in one of the following states: <code>READY_FOR_DEPLOYMENT</code>, <code>DEPLOYING</code>, <code>ROLLING_BACK</code>, or <code>ROLLED_BACK</code> </p>
    ///   - [`monitors(Option<Vec<Monitor>>)`](crate::output::GetEnvironmentOutput::monitors): <p>Amazon CloudWatch alarms monitored during the deployment.</p>
                            /// - On failure, responds with [`SdkError<GetEnvironmentError>`](crate::error::GetEnvironmentError)
    pub fn get_environment(&self) -> crate::client::fluent_builders::GetEnvironment {
                                crate::client::fluent_builders::GetEnvironment::new(self.handle.clone())
                            }
}

