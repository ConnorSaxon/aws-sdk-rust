// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSubnetGroup`](crate::client::fluent_builders::DeleteSubnetGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteSubnetGroup::subnet_group_name) / [`set_subnet_group_name(Option<String>)`](crate::client::fluent_builders::DeleteSubnetGroup::set_subnet_group_name): <p>The name of the subnet group to delete</p>
                            /// - On success, responds with [`DeleteSubnetGroupOutput`](crate::output::DeleteSubnetGroupOutput) with field(s):
    ///   - [`subnet_group(Option<SubnetGroup>)`](crate::output::DeleteSubnetGroupOutput::subnet_group): <p>The subnet group object that has been deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteSubnetGroupError>`](crate::error::DeleteSubnetGroupError)
    pub fn delete_subnet_group(&self) -> crate::client::fluent_builders::DeleteSubnetGroup {
                                crate::client::fluent_builders::DeleteSubnetGroup::new(self.handle.clone())
                            }
}

