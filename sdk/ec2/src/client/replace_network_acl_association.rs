// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ReplaceNetworkAclAssociation`](crate::client::fluent_builders::ReplaceNetworkAclAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`association_id(impl Into<String>)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::association_id) / [`set_association_id(Option<String>)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::set_association_id): <p>The ID of the current association between the original network ACL and the subnet.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`network_acl_id(impl Into<String>)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::network_acl_id) / [`set_network_acl_id(Option<String>)`](crate::client::fluent_builders::ReplaceNetworkAclAssociation::set_network_acl_id): <p>The ID of the new network ACL to associate with the subnet.</p>
                            /// - On success, responds with [`ReplaceNetworkAclAssociationOutput`](crate::output::ReplaceNetworkAclAssociationOutput) with field(s):
    ///   - [`new_association_id(Option<String>)`](crate::output::ReplaceNetworkAclAssociationOutput::new_association_id): <p>The ID of the new association.</p>
                            /// - On failure, responds with [`SdkError<ReplaceNetworkAclAssociationError>`](crate::error::ReplaceNetworkAclAssociationError)
    pub fn replace_network_acl_association(&self) -> crate::client::fluent_builders::ReplaceNetworkAclAssociation {
                                crate::client::fluent_builders::ReplaceNetworkAclAssociation::new(self.handle.clone())
                            }
}

