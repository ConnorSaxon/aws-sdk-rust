// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDBSubnetGroup`](crate::client::fluent_builders::DeleteDBSubnetGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDBSubnetGroup::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::DeleteDBSubnetGroup::set_db_subnet_group_name): <p>The name of the database subnet group to delete.</p> <note>   <p>You can't delete the default subnet group.</p>  </note>  <p>Constraints:</p>  <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p>  <p>Example: <code>mySubnetgroup</code> </p>
                            /// - On success, responds with [`DeleteDbSubnetGroupOutput`](crate::output::DeleteDbSubnetGroupOutput)
                            /// - On failure, responds with [`SdkError<DeleteDBSubnetGroupError>`](crate::error::DeleteDBSubnetGroupError)
    pub fn delete_db_subnet_group(&self) -> crate::client::fluent_builders::DeleteDBSubnetGroup {
                                crate::client::fluent_builders::DeleteDBSubnetGroup::new(self.handle.clone())
                            }
}

