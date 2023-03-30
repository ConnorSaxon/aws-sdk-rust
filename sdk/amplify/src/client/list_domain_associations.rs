// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDomainAssociations`](crate::client::fluent_builders::ListDomainAssociations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::ListDomainAssociations::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::ListDomainAssociations::set_app_id): <p> The unique ID for an Amplify app. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDomainAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDomainAssociations::set_next_token): <p> A pagination token. Set to null to start listing apps from the start. If non-null, a pagination token is returned in a result. Pass its value in here to list more projects. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDomainAssociations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListDomainAssociations::set_max_results): <p> The maximum number of records to list in a single response. </p>
                            /// - On success, responds with [`ListDomainAssociationsOutput`](crate::output::ListDomainAssociationsOutput) with field(s):
    ///   - [`domain_associations(Option<Vec<DomainAssociation>>)`](crate::output::ListDomainAssociationsOutput::domain_associations): <p> A list of domain associations. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDomainAssociationsOutput::next_token): <p> A pagination token. If non-null, a pagination token is returned in a result. Pass its value in another request to retrieve more entries. </p>
                            /// - On failure, responds with [`SdkError<ListDomainAssociationsError>`](crate::error::ListDomainAssociationsError)
    pub fn list_domain_associations(&self) -> crate::client::fluent_builders::ListDomainAssociations {
                                crate::client::fluent_builders::ListDomainAssociations::new(self.handle.clone())
                            }
}

