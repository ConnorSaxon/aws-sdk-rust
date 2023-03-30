// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSecurityConfiguration`](crate::client::fluent_builders::DeleteSecurityConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteSecurityConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteSecurityConfiguration::set_name): <p>The name of the security configuration.</p>
                            /// - On success, responds with [`DeleteSecurityConfigurationOutput`](crate::output::DeleteSecurityConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteSecurityConfigurationError>`](crate::error::DeleteSecurityConfigurationError)
    pub fn delete_security_configuration(&self) -> crate::client::fluent_builders::DeleteSecurityConfiguration {
                                crate::client::fluent_builders::DeleteSecurityConfiguration::new(self.handle.clone())
                            }
}

