// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetNode`](crate::client::fluent_builders::GetNode) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_id(impl Into<String>)`](crate::client::fluent_builders::GetNode::network_id) / [`set_network_id(Option<String>)`](crate::client::fluent_builders::GetNode::set_network_id): <p>The unique identifier of the network that the node is on.</p>
    ///   - [`member_id(impl Into<String>)`](crate::client::fluent_builders::GetNode::member_id) / [`set_member_id(Option<String>)`](crate::client::fluent_builders::GetNode::set_member_id): <p>The unique identifier of the member that owns the node.</p>  <p>Applies only to Hyperledger Fabric and is required for Hyperledger Fabric.</p>
    ///   - [`node_id(impl Into<String>)`](crate::client::fluent_builders::GetNode::node_id) / [`set_node_id(Option<String>)`](crate::client::fluent_builders::GetNode::set_node_id): <p>The unique identifier of the node.</p>
                            /// - On success, responds with [`GetNodeOutput`](crate::output::GetNodeOutput) with field(s):
    ///   - [`node(Option<Node>)`](crate::output::GetNodeOutput::node): <p>Properties of the node configuration.</p>
                            /// - On failure, responds with [`SdkError<GetNodeError>`](crate::error::GetNodeError)
    pub fn get_node(&self) -> crate::client::fluent_builders::GetNode {
                                crate::client::fluent_builders::GetNode::new(self.handle.clone())
                            }
}

