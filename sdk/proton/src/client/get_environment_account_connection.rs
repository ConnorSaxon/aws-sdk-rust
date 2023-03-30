// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEnvironmentAccountConnection`](crate::client::fluent_builders::GetEnvironmentAccountConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetEnvironmentAccountConnection::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetEnvironmentAccountConnection::set_id): <p>The ID of the environment account connection that you want to get the detailed data for.</p>
                            /// - On success, responds with [`GetEnvironmentAccountConnectionOutput`](crate::output::GetEnvironmentAccountConnectionOutput) with field(s):
    ///   - [`environment_account_connection(Option<EnvironmentAccountConnection>)`](crate::output::GetEnvironmentAccountConnectionOutput::environment_account_connection): <p>The detailed data of the requested environment account connection.</p>
                            /// - On failure, responds with [`SdkError<GetEnvironmentAccountConnectionError>`](crate::error::GetEnvironmentAccountConnectionError)
    pub fn get_environment_account_connection(&self) -> crate::client::fluent_builders::GetEnvironmentAccountConnection {
                                crate::client::fluent_builders::GetEnvironmentAccountConnection::new(self.handle.clone())
                            }
}

