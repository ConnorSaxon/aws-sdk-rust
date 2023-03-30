// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBackendAPIModels`](crate::client::fluent_builders::GetBackendAPIModels) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::GetBackendAPIModels::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::GetBackendAPIModels::set_app_id): <p>The app ID.</p>
    ///   - [`backend_environment_name(impl Into<String>)`](crate::client::fluent_builders::GetBackendAPIModels::backend_environment_name) / [`set_backend_environment_name(Option<String>)`](crate::client::fluent_builders::GetBackendAPIModels::set_backend_environment_name): <p>The name of the backend environment.</p>
    ///   - [`resource_name(impl Into<String>)`](crate::client::fluent_builders::GetBackendAPIModels::resource_name) / [`set_resource_name(Option<String>)`](crate::client::fluent_builders::GetBackendAPIModels::set_resource_name): <p>The name of this resource.</p>
                            /// - On success, responds with [`GetBackendApiModelsOutput`](crate::output::GetBackendApiModelsOutput) with field(s):
    ///   - [`models(Option<String>)`](crate::output::GetBackendApiModelsOutput::models): <p>Stringified JSON of the datastore model.</p>
    ///   - [`status(Option<Status>)`](crate::output::GetBackendApiModelsOutput::status): <p>The current status of the request.</p>
    ///   - [`model_introspection_schema(Option<String>)`](crate::output::GetBackendApiModelsOutput::model_introspection_schema): <p>Stringified JSON of the model introspection schema for an existing backend API resource.</p>
                            /// - On failure, responds with [`SdkError<GetBackendAPIModelsError>`](crate::error::GetBackendAPIModelsError)
    pub fn get_backend_api_models(&self) -> crate::client::fluent_builders::GetBackendAPIModels {
                                crate::client::fluent_builders::GetBackendAPIModels::new(self.handle.clone())
                            }
}

