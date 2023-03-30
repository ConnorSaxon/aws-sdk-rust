// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResolverRuleAssociation`](crate::client::fluent_builders::GetResolverRuleAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resolver_rule_association_id(impl Into<String>)`](crate::client::fluent_builders::GetResolverRuleAssociation::resolver_rule_association_id) / [`set_resolver_rule_association_id(Option<String>)`](crate::client::fluent_builders::GetResolverRuleAssociation::set_resolver_rule_association_id): <p>The ID of the Resolver rule association that you want to get information about.</p>
                            /// - On success, responds with [`GetResolverRuleAssociationOutput`](crate::output::GetResolverRuleAssociationOutput) with field(s):
    ///   - [`resolver_rule_association(Option<ResolverRuleAssociation>)`](crate::output::GetResolverRuleAssociationOutput::resolver_rule_association): <p>Information about the Resolver rule association that you specified in a <code>GetResolverRuleAssociation</code> request.</p>
                            /// - On failure, responds with [`SdkError<GetResolverRuleAssociationError>`](crate::error::GetResolverRuleAssociationError)
    pub fn get_resolver_rule_association(&self) -> crate::client::fluent_builders::GetResolverRuleAssociation {
                                crate::client::fluent_builders::GetResolverRuleAssociation::new(self.handle.clone())
                            }
}

