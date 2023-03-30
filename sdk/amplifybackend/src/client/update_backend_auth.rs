// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateBackendAuth`](crate::client::fluent_builders::UpdateBackendAuth) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::UpdateBackendAuth::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::UpdateBackendAuth::set_app_id): <p>The app ID.</p>
    ///   - [`backend_environment_name(impl Into<String>)`](crate::client::fluent_builders::UpdateBackendAuth::backend_environment_name) / [`set_backend_environment_name(Option<String>)`](crate::client::fluent_builders::UpdateBackendAuth::set_backend_environment_name): <p>The name of the backend environment.</p>
    ///   - [`resource_config(UpdateBackendAuthResourceConfig)`](crate::client::fluent_builders::UpdateBackendAuth::resource_config) / [`set_resource_config(Option<UpdateBackendAuthResourceConfig>)`](crate::client::fluent_builders::UpdateBackendAuth::set_resource_config): <p>The resource configuration for this request object.</p>
    ///   - [`resource_name(impl Into<String>)`](crate::client::fluent_builders::UpdateBackendAuth::resource_name) / [`set_resource_name(Option<String>)`](crate::client::fluent_builders::UpdateBackendAuth::set_resource_name): <p>The name of this resource.</p>
                            /// - On success, responds with [`UpdateBackendAuthOutput`](crate::output::UpdateBackendAuthOutput) with field(s):
    ///   - [`app_id(Option<String>)`](crate::output::UpdateBackendAuthOutput::app_id): <p>The app ID.</p>
    ///   - [`backend_environment_name(Option<String>)`](crate::output::UpdateBackendAuthOutput::backend_environment_name): <p>The name of the backend environment.</p>
    ///   - [`error(Option<String>)`](crate::output::UpdateBackendAuthOutput::error): <p>If the request fails, this error is returned.</p>
    ///   - [`job_id(Option<String>)`](crate::output::UpdateBackendAuthOutput::job_id): <p>The ID for the job.</p>
    ///   - [`operation(Option<String>)`](crate::output::UpdateBackendAuthOutput::operation): <p>The name of the operation.</p>
    ///   - [`status(Option<String>)`](crate::output::UpdateBackendAuthOutput::status): <p>The current status of the request.</p>
                            /// - On failure, responds with [`SdkError<UpdateBackendAuthError>`](crate::error::UpdateBackendAuthError)
    pub fn update_backend_auth(&self) -> crate::client::fluent_builders::UpdateBackendAuth {
                                crate::client::fluent_builders::UpdateBackendAuth::new(self.handle.clone())
                            }
}

