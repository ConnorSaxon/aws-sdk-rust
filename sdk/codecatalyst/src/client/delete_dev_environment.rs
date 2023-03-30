// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDevEnvironment`](crate::client::fluent_builders::DeleteDevEnvironment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`space_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::space_name) / [`set_space_name(Option<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::set_space_name): <p>The name of the space.</p>
    ///   - [`project_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::project_name) / [`set_project_name(Option<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::set_project_name): <p>The name of the project in the space.</p>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteDevEnvironment::set_id): <p>The system-generated unique ID of the Dev Environment you want to delete. To retrieve a list of Dev Environment IDs, use <code>ListDevEnvironments</code>.</p>
                            /// - On success, responds with [`DeleteDevEnvironmentOutput`](crate::output::DeleteDevEnvironmentOutput) with field(s):
    ///   - [`space_name(Option<String>)`](crate::output::DeleteDevEnvironmentOutput::space_name): <p>The name of the space.</p>
    ///   - [`project_name(Option<String>)`](crate::output::DeleteDevEnvironmentOutput::project_name): <p>The name of the project in the space.</p>
    ///   - [`id(Option<String>)`](crate::output::DeleteDevEnvironmentOutput::id): <p>The system-generated unique ID of the deleted Dev Environment. </p>
                            /// - On failure, responds with [`SdkError<DeleteDevEnvironmentError>`](crate::error::DeleteDevEnvironmentError)
    pub fn delete_dev_environment(&self) -> crate::client::fluent_builders::DeleteDevEnvironment {
                                crate::client::fluent_builders::DeleteDevEnvironment::new(self.handle.clone())
                            }
}

