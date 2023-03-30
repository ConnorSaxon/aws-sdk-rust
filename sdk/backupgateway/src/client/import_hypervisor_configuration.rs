// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ImportHypervisorConfiguration`](crate::client::fluent_builders::ImportHypervisorConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_name): <p>The name of the hypervisor.</p>
    ///   - [`host(impl Into<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::host) / [`set_host(Option<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_host): <p>The server host of the hypervisor. This can be either an IP address or a fully-qualified domain name (FQDN).</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_username): <p>The username for the hypervisor.</p>
    ///   - [`password(impl Into<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::password) / [`set_password(Option<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_password): <p>The password for the hypervisor.</p>
    ///   - [`kms_key_arn(impl Into<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::kms_key_arn) / [`set_kms_key_arn(Option<String>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_kms_key_arn): <p>The Key Management Service for the hypervisor.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::ImportHypervisorConfiguration::set_tags): <p>The tags of the hypervisor configuration to import.</p>
                            /// - On success, responds with [`ImportHypervisorConfigurationOutput`](crate::output::ImportHypervisorConfigurationOutput) with field(s):
    ///   - [`hypervisor_arn(Option<String>)`](crate::output::ImportHypervisorConfigurationOutput::hypervisor_arn): <p>The Amazon Resource Name (ARN) of the hypervisor you disassociated.</p>
                            /// - On failure, responds with [`SdkError<ImportHypervisorConfigurationError>`](crate::error::ImportHypervisorConfigurationError)
    pub fn import_hypervisor_configuration(&self) -> crate::client::fluent_builders::ImportHypervisorConfiguration {
                                crate::client::fluent_builders::ImportHypervisorConfiguration::new(self.handle.clone())
                            }
}

