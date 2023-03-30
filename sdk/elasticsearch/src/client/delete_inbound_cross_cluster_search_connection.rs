// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteInboundCrossClusterSearchConnection`](crate::client::fluent_builders::DeleteInboundCrossClusterSearchConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cross_cluster_search_connection_id(impl Into<String>)`](crate::client::fluent_builders::DeleteInboundCrossClusterSearchConnection::cross_cluster_search_connection_id) / [`set_cross_cluster_search_connection_id(Option<String>)`](crate::client::fluent_builders::DeleteInboundCrossClusterSearchConnection::set_cross_cluster_search_connection_id): <p>The id of the inbound connection that you want to permanently delete.</p>
                            /// - On success, responds with [`DeleteInboundCrossClusterSearchConnectionOutput`](crate::output::DeleteInboundCrossClusterSearchConnectionOutput) with field(s):
    ///   - [`cross_cluster_search_connection(Option<InboundCrossClusterSearchConnection>)`](crate::output::DeleteInboundCrossClusterSearchConnectionOutput::cross_cluster_search_connection): <p>Specifies the <code><code>InboundCrossClusterSearchConnection</code></code> of deleted inbound connection. </p>
                            /// - On failure, responds with [`SdkError<DeleteInboundCrossClusterSearchConnectionError>`](crate::error::DeleteInboundCrossClusterSearchConnectionError)
    pub fn delete_inbound_cross_cluster_search_connection(&self) -> crate::client::fluent_builders::DeleteInboundCrossClusterSearchConnection {
                                crate::client::fluent_builders::DeleteInboundCrossClusterSearchConnection::new(self.handle.clone())
                            }
}

