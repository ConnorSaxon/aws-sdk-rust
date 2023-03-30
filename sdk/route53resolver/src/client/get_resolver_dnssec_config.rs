// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResolverDnssecConfig`](crate::client::fluent_builders::GetResolverDnssecConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::GetResolverDnssecConfig::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::GetResolverDnssecConfig::set_resource_id): <p>The ID of the virtual private cloud (VPC) for the DNSSEC validation status.</p>
                            /// - On success, responds with [`GetResolverDnssecConfigOutput`](crate::output::GetResolverDnssecConfigOutput) with field(s):
    ///   - [`resolver_dnssec_config(Option<ResolverDnssecConfig>)`](crate::output::GetResolverDnssecConfigOutput::resolver_dnssec_config): <p>The information about a configuration for DNSSEC validation.</p>
                            /// - On failure, responds with [`SdkError<GetResolverDnssecConfigError>`](crate::error::GetResolverDnssecConfigError)
    pub fn get_resolver_dnssec_config(&self) -> crate::client::fluent_builders::GetResolverDnssecConfig {
                                crate::client::fluent_builders::GetResolverDnssecConfig::new(self.handle.clone())
                            }
}

