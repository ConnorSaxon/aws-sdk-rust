// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDomainNames`](crate::client::fluent_builders::ListDomainNames) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`engine_type(EngineType)`](crate::client::fluent_builders::ListDomainNames::engine_type) / [`set_engine_type(Option<EngineType>)`](crate::client::fluent_builders::ListDomainNames::set_engine_type): <p>Filters the output by domain engine type.</p>
                            /// - On success, responds with [`ListDomainNamesOutput`](crate::output::ListDomainNamesOutput) with field(s):
    ///   - [`domain_names(Option<Vec<DomainInfo>>)`](crate::output::ListDomainNamesOutput::domain_names): <p>The names of all OpenSearch Service domains owned by the current user and their respective engine types.</p>
                            /// - On failure, responds with [`SdkError<ListDomainNamesError>`](crate::error::ListDomainNamesError)
    pub fn list_domain_names(&self) -> crate::client::fluent_builders::ListDomainNames {
                                crate::client::fluent_builders::ListDomainNames::new(self.handle.clone())
                            }
}

