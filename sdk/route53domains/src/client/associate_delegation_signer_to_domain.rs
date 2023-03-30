// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateDelegationSignerToDomain`](crate::client::fluent_builders::AssociateDelegationSignerToDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::AssociateDelegationSignerToDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::AssociateDelegationSignerToDomain::set_domain_name): <p>The name of the domain.</p>
    ///   - [`signing_attributes(DnssecSigningAttributes)`](crate::client::fluent_builders::AssociateDelegationSignerToDomain::signing_attributes) / [`set_signing_attributes(Option<DnssecSigningAttributes>)`](crate::client::fluent_builders::AssociateDelegationSignerToDomain::set_signing_attributes): <p>The information about a key, including the algorithm, public key-value, and flags.</p>
                            /// - On success, responds with [`AssociateDelegationSignerToDomainOutput`](crate::output::AssociateDelegationSignerToDomainOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::output::AssociateDelegationSignerToDomainOutput::operation_id): <p>The identifier for tracking the progress of the request. To query the operation status, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_GetOperationDetail.html">GetOperationDetail</a>.</p>
                            /// - On failure, responds with [`SdkError<AssociateDelegationSignerToDomainError>`](crate::error::AssociateDelegationSignerToDomainError)
    pub fn associate_delegation_signer_to_domain(&self) -> crate::client::fluent_builders::AssociateDelegationSignerToDomain {
                                crate::client::fluent_builders::AssociateDelegationSignerToDomain::new(self.handle.clone())
                            }
}

