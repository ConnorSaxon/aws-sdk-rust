// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteLoggingConfiguration`](crate::client::fluent_builders::DeleteLoggingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteLoggingConfiguration::identifier) / [`set_identifier(Option<String>)`](crate::client::fluent_builders::DeleteLoggingConfiguration::set_identifier): <p>Identifier of the logging configuration to be deleted.</p>
                            /// - On success, responds with [`DeleteLoggingConfigurationOutput`](crate::output::DeleteLoggingConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteLoggingConfigurationError>`](crate::error::DeleteLoggingConfigurationError)
    pub fn delete_logging_configuration(&self) -> crate::client::fluent_builders::DeleteLoggingConfiguration {
                                crate::client::fluent_builders::DeleteLoggingConfiguration::new(self.handle.clone())
                            }
}

