// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDomain`](crate::client::fluent_builders::DeleteDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DeleteDomain::set_domain_name): <p>The name of the domain you want to permanently delete.</p>
                            /// - On success, responds with [`DeleteDomainOutput`](crate::output::DeleteDomainOutput) with field(s):
    ///   - [`domain_status(Option<DomainStatus>)`](crate::output::DeleteDomainOutput::domain_status): <p>The current status of the search domain.</p>
                            /// - On failure, responds with [`SdkError<DeleteDomainError>`](crate::error::DeleteDomainError)
    pub fn delete_domain(&self) -> crate::client::fluent_builders::DeleteDomain {
                                crate::client::fluent_builders::DeleteDomain::new(self.handle.clone())
                            }
}

