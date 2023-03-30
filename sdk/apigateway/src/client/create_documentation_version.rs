// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDocumentationVersion`](crate::client::fluent_builders::CreateDocumentationVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`documentation_version(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::documentation_version) / [`set_documentation_version(Option<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::set_documentation_version): <p>The version identifier of the new snapshot.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::set_stage_name): <p>The stage name to be associated with the new documentation snapshot.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateDocumentationVersion::set_description): <p>A description about the new documentation snapshot.</p>
                            /// - On success, responds with [`CreateDocumentationVersionOutput`](crate::output::CreateDocumentationVersionOutput) with field(s):
    ///   - [`version(Option<String>)`](crate::output::CreateDocumentationVersionOutput::version): <p>The version identifier of the API documentation snapshot.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::output::CreateDocumentationVersionOutput::created_date): <p>The date when the API documentation snapshot is created.</p>
    ///   - [`description(Option<String>)`](crate::output::CreateDocumentationVersionOutput::description): <p>The description of the API documentation snapshot.</p>
                            /// - On failure, responds with [`SdkError<CreateDocumentationVersionError>`](crate::error::CreateDocumentationVersionError)
    pub fn create_documentation_version(&self) -> crate::client::fluent_builders::CreateDocumentationVersion {
                                crate::client::fluent_builders::CreateDocumentationVersion::new(self.handle.clone())
                            }
}

