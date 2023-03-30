// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAppValidationConfiguration`](crate::client::fluent_builders::DeleteAppValidationConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAppValidationConfiguration::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::DeleteAppValidationConfiguration::set_app_id): <p>The ID of the application.</p>
                            /// - On success, responds with [`DeleteAppValidationConfigurationOutput`](crate::output::DeleteAppValidationConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteAppValidationConfigurationError>`](crate::error::DeleteAppValidationConfigurationError)
    pub fn delete_app_validation_configuration(&self) -> crate::client::fluent_builders::DeleteAppValidationConfiguration {
                                crate::client::fluent_builders::DeleteAppValidationConfiguration::new(self.handle.clone())
                            }
}

