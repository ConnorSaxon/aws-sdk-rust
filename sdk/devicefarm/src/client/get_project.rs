// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetProject`](crate::client::fluent_builders::GetProject) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetProject::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetProject::set_arn): <p>The project's ARN.</p>
                            /// - On success, responds with [`GetProjectOutput`](crate::output::GetProjectOutput) with field(s):
    ///   - [`project(Option<Project>)`](crate::output::GetProjectOutput::project): <p>The project to get information about.</p>
                            /// - On failure, responds with [`SdkError<GetProjectError>`](crate::error::GetProjectError)
    pub fn get_project(&self) -> crate::client::fluent_builders::GetProject {
                                crate::client::fluent_builders::GetProject::new(self.handle.clone())
                            }
}

