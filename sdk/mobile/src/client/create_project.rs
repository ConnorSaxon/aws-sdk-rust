// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateProject`](crate::client::fluent_builders::CreateProject) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateProject::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateProject::set_name): <p> Name of the project. </p>
    ///   - [`region(impl Into<String>)`](crate::client::fluent_builders::CreateProject::region) / [`set_region(Option<String>)`](crate::client::fluent_builders::CreateProject::set_region): <p> Default region where project resources should be created. </p>
    ///   - [`contents(Blob)`](crate::client::fluent_builders::CreateProject::contents) / [`set_contents(Option<Blob>)`](crate::client::fluent_builders::CreateProject::set_contents): <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    ///   - [`snapshot_id(impl Into<String>)`](crate::client::fluent_builders::CreateProject::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::client::fluent_builders::CreateProject::set_snapshot_id): <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
                            /// - On success, responds with [`CreateProjectOutput`](crate::output::CreateProjectOutput) with field(s):
    ///   - [`details(Option<ProjectDetails>)`](crate::output::CreateProjectOutput::details): <p> Detailed information about the created AWS Mobile Hub project. </p>
                            /// - On failure, responds with [`SdkError<CreateProjectError>`](crate::error::CreateProjectError)
    pub fn create_project(&self) -> crate::client::fluent_builders::CreateProject {
                                crate::client::fluent_builders::CreateProject::new(self.handle.clone())
                            }
}

