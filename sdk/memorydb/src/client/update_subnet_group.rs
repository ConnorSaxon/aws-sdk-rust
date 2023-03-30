// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSubnetGroup`](crate::client::fluent_builders::UpdateSubnetGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::UpdateSubnetGroup::subnet_group_name) / [`set_subnet_group_name(Option<String>)`](crate::client::fluent_builders::UpdateSubnetGroup::set_subnet_group_name): <p>The name of the subnet group</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateSubnetGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateSubnetGroup::set_description): <p>A description of the subnet group</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::UpdateSubnetGroup::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateSubnetGroup::set_subnet_ids): <p>The EC2 subnet IDs for the subnet group.</p>
                            /// - On success, responds with [`UpdateSubnetGroupOutput`](crate::output::UpdateSubnetGroupOutput) with field(s):
    ///   - [`subnet_group(Option<SubnetGroup>)`](crate::output::UpdateSubnetGroupOutput::subnet_group): <p>The updated subnet group</p>
                            /// - On failure, responds with [`SdkError<UpdateSubnetGroupError>`](crate::error::UpdateSubnetGroupError)
    pub fn update_subnet_group(&self) -> crate::client::fluent_builders::UpdateSubnetGroup {
                                crate::client::fluent_builders::UpdateSubnetGroup::new(self.handle.clone())
                            }
}

