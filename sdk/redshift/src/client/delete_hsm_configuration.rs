// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteHsmConfiguration`](crate::client::fluent_builders::DeleteHsmConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hsm_configuration_identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteHsmConfiguration::hsm_configuration_identifier) / [`set_hsm_configuration_identifier(Option<String>)`](crate::client::fluent_builders::DeleteHsmConfiguration::set_hsm_configuration_identifier): <p>The identifier of the Amazon Redshift HSM configuration to be deleted.</p>
                            /// - On success, responds with [`DeleteHsmConfigurationOutput`](crate::output::DeleteHsmConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteHsmConfigurationError>`](crate::error::DeleteHsmConfigurationError)
    pub fn delete_hsm_configuration(&self) -> crate::client::fluent_builders::DeleteHsmConfiguration {
                                crate::client::fluent_builders::DeleteHsmConfiguration::new(self.handle.clone())
                            }
}

