// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteProject`](crate::client::fluent_builders::DeleteProject) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_name(impl Into<String>)`](crate::client::fluent_builders::DeleteProject::project_name) / [`set_project_name(Option<String>)`](crate::client::fluent_builders::DeleteProject::set_project_name): <p>The name of the empty project to delete.</p>
                            /// - On success, responds with [`DeleteProjectOutput`](crate::output::DeleteProjectOutput)
                            /// - On failure, responds with [`SdkError<DeleteProjectError>`](crate::error::DeleteProjectError)
    pub fn delete_project(&self) -> crate::client::fluent_builders::DeleteProject {
                                crate::client::fluent_builders::DeleteProject::new(self.handle.clone())
                            }
}

