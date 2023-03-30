// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyAquaConfiguration`](crate::client::fluent_builders::ModifyAquaConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyAquaConfiguration::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::ModifyAquaConfiguration::set_cluster_identifier): <p>The identifier of the cluster to be modified.</p>
    ///   - [`aqua_configuration_status(AquaConfigurationStatus)`](crate::client::fluent_builders::ModifyAquaConfiguration::aqua_configuration_status) / [`set_aqua_configuration_status(Option<AquaConfigurationStatus>)`](crate::client::fluent_builders::ModifyAquaConfiguration::set_aqua_configuration_status): <p>This parameter is retired. Amazon Redshift automatically determines whether to use AQUA (Advanced Query Accelerator).</p>
                            /// - On success, responds with [`ModifyAquaConfigurationOutput`](crate::output::ModifyAquaConfigurationOutput) with field(s):
    ///   - [`aqua_configuration(Option<AquaConfiguration>)`](crate::output::ModifyAquaConfigurationOutput::aqua_configuration): <p>This parameter is retired. Amazon Redshift automatically determines whether to use AQUA (Advanced Query Accelerator). </p>
                            /// - On failure, responds with [`SdkError<ModifyAquaConfigurationError>`](crate::error::ModifyAquaConfigurationError)
    pub fn modify_aqua_configuration(&self) -> crate::client::fluent_builders::ModifyAquaConfiguration {
                                crate::client::fluent_builders::ModifyAquaConfiguration::new(self.handle.clone())
                            }
}

