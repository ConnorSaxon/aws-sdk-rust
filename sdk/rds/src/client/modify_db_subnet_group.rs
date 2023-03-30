// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyDBSubnetGroup`](crate::client::fluent_builders::ModifyDBSubnetGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::set_db_subnet_group_name): <p>The name for the DB subnet group. This value is stored as a lowercase string. You can't modify the default subnet group.</p>  <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p>  <p>Example: <code>mydbsubnetgroup</code> </p>
    ///   - [`db_subnet_group_description(impl Into<String>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::db_subnet_group_description) / [`set_db_subnet_group_description(Option<String>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::set_db_subnet_group_description): <p>The description for the DB subnet group.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyDBSubnetGroup::set_subnet_ids): <p>The EC2 subnet IDs for the DB subnet group.</p>
                            /// - On success, responds with [`ModifyDbSubnetGroupOutput`](crate::output::ModifyDbSubnetGroupOutput) with field(s):
    ///   - [`db_subnet_group(Option<DbSubnetGroup>)`](crate::output::ModifyDbSubnetGroupOutput::db_subnet_group): <p>Contains the details of an Amazon RDS DB subnet group.</p>  <p>This data type is used as a response element in the <code>DescribeDBSubnetGroups</code> action.</p>
                            /// - On failure, responds with [`SdkError<ModifyDBSubnetGroupError>`](crate::error::ModifyDBSubnetGroupError)
    pub fn modify_db_subnet_group(&self) -> crate::client::fluent_builders::ModifyDBSubnetGroup {
                                crate::client::fluent_builders::ModifyDBSubnetGroup::new(self.handle.clone())
                            }
}

