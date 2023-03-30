// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RebootNode`](crate::client::fluent_builders::RebootNode) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::client::fluent_builders::RebootNode::cluster_name) / [`set_cluster_name(Option<String>)`](crate::client::fluent_builders::RebootNode::set_cluster_name): <p>The name of the DAX cluster containing the node to be rebooted.</p>
    ///   - [`node_id(impl Into<String>)`](crate::client::fluent_builders::RebootNode::node_id) / [`set_node_id(Option<String>)`](crate::client::fluent_builders::RebootNode::set_node_id): <p>The system-assigned ID of the node to be rebooted.</p>
                            /// - On success, responds with [`RebootNodeOutput`](crate::output::RebootNodeOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::RebootNodeOutput::cluster): <p>A description of the DAX cluster after a node has been rebooted.</p>
                            /// - On failure, responds with [`SdkError<RebootNodeError>`](crate::error::RebootNodeError)
    pub fn reboot_node(&self) -> crate::client::fluent_builders::RebootNode {
                                crate::client::fluent_builders::RebootNode::new(self.handle.clone())
                            }
}

