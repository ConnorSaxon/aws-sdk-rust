// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RejectInboundConnection`](crate::client::fluent_builders::RejectInboundConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`connection_id(impl Into<String>)`](crate::client::fluent_builders::RejectInboundConnection::connection_id) / [`set_connection_id(Option<String>)`](crate::client::fluent_builders::RejectInboundConnection::set_connection_id): <p>The unique identifier of the inbound connection to reject.</p>
                            /// - On success, responds with [`RejectInboundConnectionOutput`](crate::output::RejectInboundConnectionOutput) with field(s):
    ///   - [`connection(Option<InboundConnection>)`](crate::output::RejectInboundConnectionOutput::connection): <p>Contains details about the rejected inbound connection.</p>
                            /// - On failure, responds with [`SdkError<RejectInboundConnectionError>`](crate::error::RejectInboundConnectionError)
    pub fn reject_inbound_connection(&self) -> crate::client::fluent_builders::RejectInboundConnection {
                                crate::client::fluent_builders::RejectInboundConnection::new(self.handle.clone())
                            }
}

