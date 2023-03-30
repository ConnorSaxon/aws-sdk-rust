// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteOutboundCrossClusterSearchConnection`](crate::client::fluent_builders::DeleteOutboundCrossClusterSearchConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cross_cluster_search_connection_id(impl Into<String>)`](crate::client::fluent_builders::DeleteOutboundCrossClusterSearchConnection::cross_cluster_search_connection_id) / [`set_cross_cluster_search_connection_id(Option<String>)`](crate::client::fluent_builders::DeleteOutboundCrossClusterSearchConnection::set_cross_cluster_search_connection_id): <p>The id of the outbound connection that you want to permanently delete.</p>
                            /// - On success, responds with [`DeleteOutboundCrossClusterSearchConnectionOutput`](crate::output::DeleteOutboundCrossClusterSearchConnectionOutput) with field(s):
    ///   - [`cross_cluster_search_connection(Option<OutboundCrossClusterSearchConnection>)`](crate::output::DeleteOutboundCrossClusterSearchConnectionOutput::cross_cluster_search_connection): <p>Specifies the <code><code>OutboundCrossClusterSearchConnection</code></code> of deleted outbound connection. </p>
                            /// - On failure, responds with [`SdkError<DeleteOutboundCrossClusterSearchConnectionError>`](crate::error::DeleteOutboundCrossClusterSearchConnectionError)
    pub fn delete_outbound_cross_cluster_search_connection(&self) -> crate::client::fluent_builders::DeleteOutboundCrossClusterSearchConnection {
                                crate::client::fluent_builders::DeleteOutboundCrossClusterSearchConnection::new(self.handle.clone())
                            }
}

