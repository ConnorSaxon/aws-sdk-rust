// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteQueryLoggingConfig`](crate::client::fluent_builders::DeleteQueryLoggingConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteQueryLoggingConfig::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteQueryLoggingConfig::set_id): <p>The ID of the configuration that you want to delete. </p>
                            /// - On success, responds with [`DeleteQueryLoggingConfigOutput`](crate::output::DeleteQueryLoggingConfigOutput)
                            /// - On failure, responds with [`SdkError<DeleteQueryLoggingConfigError>`](crate::error::DeleteQueryLoggingConfigError)
    pub fn delete_query_logging_config(&self) -> crate::client::fluent_builders::DeleteQueryLoggingConfig {
                                crate::client::fluent_builders::DeleteQueryLoggingConfig::new(self.handle.clone())
                            }
}

