// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateConnectPeer`](crate::client::fluent_builders::DisassociateConnectPeer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateConnectPeer::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::DisassociateConnectPeer::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`connect_peer_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateConnectPeer::connect_peer_id) / [`set_connect_peer_id(Option<String>)`](crate::client::fluent_builders::DisassociateConnectPeer::set_connect_peer_id): <p>The ID of the Connect peer to disassociate from a device.</p>
                            /// - On success, responds with [`DisassociateConnectPeerOutput`](crate::output::DisassociateConnectPeerOutput) with field(s):
    ///   - [`connect_peer_association(Option<ConnectPeerAssociation>)`](crate::output::DisassociateConnectPeerOutput::connect_peer_association): <p>Describes the Connect peer association.</p>
                            /// - On failure, responds with [`SdkError<DisassociateConnectPeerError>`](crate::error::DisassociateConnectPeerError)
    pub fn disassociate_connect_peer(&self) -> crate::client::fluent_builders::DisassociateConnectPeer {
                                crate::client::fluent_builders::DisassociateConnectPeer::new(self.handle.clone())
                            }
}

