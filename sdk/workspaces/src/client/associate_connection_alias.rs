// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateConnectionAlias`](crate::client::fluent_builders::AssociateConnectionAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alias_id(impl Into<String>)`](crate::client::fluent_builders::AssociateConnectionAlias::alias_id) / [`set_alias_id(Option<String>)`](crate::client::fluent_builders::AssociateConnectionAlias::set_alias_id): <p>The identifier of the connection alias.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::AssociateConnectionAlias::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::AssociateConnectionAlias::set_resource_id): <p>The identifier of the directory to associate the connection alias with.</p>
                            /// - On success, responds with [`AssociateConnectionAliasOutput`](crate::output::AssociateConnectionAliasOutput) with field(s):
    ///   - [`connection_identifier(Option<String>)`](crate::output::AssociateConnectionAliasOutput::connection_identifier): <p>The identifier of the connection alias association. You use the connection identifier in the DNS TXT record when you're configuring your DNS routing policies. </p>
                            /// - On failure, responds with [`SdkError<AssociateConnectionAliasError>`](crate::error::AssociateConnectionAliasError)
    pub fn associate_connection_alias(&self) -> crate::client::fluent_builders::AssociateConnectionAlias {
                                crate::client::fluent_builders::AssociateConnectionAlias::new(self.handle.clone())
                            }
}

