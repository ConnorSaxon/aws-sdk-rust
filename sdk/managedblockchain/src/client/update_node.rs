// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateNode`](crate::client::fluent_builders::UpdateNode) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_id(impl Into<String>)`](crate::client::fluent_builders::UpdateNode::network_id) / [`set_network_id(Option<String>)`](crate::client::fluent_builders::UpdateNode::set_network_id): <p>The unique identifier of the network that the node is on.</p>
    ///   - [`member_id(impl Into<String>)`](crate::client::fluent_builders::UpdateNode::member_id) / [`set_member_id(Option<String>)`](crate::client::fluent_builders::UpdateNode::set_member_id): <p>The unique identifier of the member that owns the node.</p>  <p>Applies only to Hyperledger Fabric.</p>
    ///   - [`node_id(impl Into<String>)`](crate::client::fluent_builders::UpdateNode::node_id) / [`set_node_id(Option<String>)`](crate::client::fluent_builders::UpdateNode::set_node_id): <p>The unique identifier of the node.</p>
    ///   - [`log_publishing_configuration(NodeLogPublishingConfiguration)`](crate::client::fluent_builders::UpdateNode::log_publishing_configuration) / [`set_log_publishing_configuration(Option<NodeLogPublishingConfiguration>)`](crate::client::fluent_builders::UpdateNode::set_log_publishing_configuration): <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
                            /// - On success, responds with [`UpdateNodeOutput`](crate::output::UpdateNodeOutput)
                            /// - On failure, responds with [`SdkError<UpdateNodeError>`](crate::error::UpdateNodeError)
    pub fn update_node(&self) -> crate::client::fluent_builders::UpdateNode {
                                crate::client::fluent_builders::UpdateNode::new(self.handle.clone())
                            }
}

