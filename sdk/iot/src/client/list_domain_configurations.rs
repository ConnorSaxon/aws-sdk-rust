// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDomainConfigurations`](crate::client::fluent_builders::ListDomainConfigurations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDomainConfigurations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListDomainConfigurations::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListDomainConfigurations::set_marker): <p>The marker for the next set of results.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListDomainConfigurations::page_size) / [`set_page_size(Option<i32>)`](crate::client::fluent_builders::ListDomainConfigurations::set_page_size): <p>The result page size.</p>
    ///   - [`service_type(ServiceType)`](crate::client::fluent_builders::ListDomainConfigurations::service_type) / [`set_service_type(Option<ServiceType>)`](crate::client::fluent_builders::ListDomainConfigurations::set_service_type): <p>The type of service delivered by the endpoint.</p>
                            /// - On success, responds with [`ListDomainConfigurationsOutput`](crate::output::ListDomainConfigurationsOutput) with field(s):
    ///   - [`domain_configurations(Option<Vec<DomainConfigurationSummary>>)`](crate::output::ListDomainConfigurationsOutput::domain_configurations): <p>A list of objects that contain summary information about the user's domain configurations.</p>
    ///   - [`next_marker(Option<String>)`](crate::output::ListDomainConfigurationsOutput::next_marker): <p>The marker for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListDomainConfigurationsError>`](crate::error::ListDomainConfigurationsError)
    pub fn list_domain_configurations(&self) -> crate::client::fluent_builders::ListDomainConfigurations {
                                crate::client::fluent_builders::ListDomainConfigurations::new(self.handle.clone())
                            }
}

