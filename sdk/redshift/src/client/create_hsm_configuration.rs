// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateHsmConfiguration`](crate::client::fluent_builders::CreateHsmConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hsm_configuration_identifier(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::hsm_configuration_identifier) / [`set_hsm_configuration_identifier(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_hsm_configuration_identifier): <p>The identifier to be assigned to the new Amazon Redshift HSM configuration.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_description): <p>A text description of the HSM configuration to be created.</p>
    ///   - [`hsm_ip_address(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::hsm_ip_address) / [`set_hsm_ip_address(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_hsm_ip_address): <p>The IP address that the Amazon Redshift cluster must use to access the HSM.</p>
    ///   - [`hsm_partition_name(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::hsm_partition_name) / [`set_hsm_partition_name(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_hsm_partition_name): <p>The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.</p>
    ///   - [`hsm_partition_password(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::hsm_partition_password) / [`set_hsm_partition_password(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_hsm_partition_password): <p>The password required to access the HSM partition.</p>
    ///   - [`hsm_server_public_certificate(impl Into<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::hsm_server_public_certificate) / [`set_hsm_server_public_certificate(Option<String>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_hsm_server_public_certificate): <p>The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateHsmConfiguration::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateHsmConfiguration::set_tags): <p>A list of tag instances.</p>
                            /// - On success, responds with [`CreateHsmConfigurationOutput`](crate::output::CreateHsmConfigurationOutput) with field(s):
    ///   - [`hsm_configuration(Option<HsmConfiguration>)`](crate::output::CreateHsmConfigurationOutput::hsm_configuration): <p>Returns information about an HSM configuration, which is an object that describes to Amazon Redshift clusters the information they require to connect to an HSM where they can store database encryption keys.</p>
                            /// - On failure, responds with [`SdkError<CreateHsmConfigurationError>`](crate::error::CreateHsmConfigurationError)
    pub fn create_hsm_configuration(&self) -> crate::client::fluent_builders::CreateHsmConfiguration {
                                crate::client::fluent_builders::CreateHsmConfiguration::new(self.handle.clone())
                            }
}

