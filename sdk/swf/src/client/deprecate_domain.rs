// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeprecateDomain`](crate::client::fluent_builders::DeprecateDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeprecateDomain::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeprecateDomain::set_name): <p>The name of the domain to deprecate.</p>
                            /// - On success, responds with [`DeprecateDomainOutput`](crate::output::DeprecateDomainOutput)
                            /// - On failure, responds with [`SdkError<DeprecateDomainError>`](crate::error::DeprecateDomainError)
    pub fn deprecate_domain(&self) -> crate::client::fluent_builders::DeprecateDomain {
                                crate::client::fluent_builders::DeprecateDomain::new(self.handle.clone())
                            }
}

