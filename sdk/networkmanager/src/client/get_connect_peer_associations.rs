// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetConnectPeerAssociations`](crate::client::fluent_builders::GetConnectPeerAssociations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetConnectPeerAssociations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::GetConnectPeerAssociations::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::GetConnectPeerAssociations::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`connect_peer_ids(Vec<String>)`](crate::client::fluent_builders::GetConnectPeerAssociations::connect_peer_ids) / [`set_connect_peer_ids(Option<Vec<String>>)`](crate::client::fluent_builders::GetConnectPeerAssociations::set_connect_peer_ids): <p>The IDs of the Connect peers.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetConnectPeerAssociations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetConnectPeerAssociations::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetConnectPeerAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetConnectPeerAssociations::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`GetConnectPeerAssociationsOutput`](crate::output::GetConnectPeerAssociationsOutput) with field(s):
    ///   - [`connect_peer_associations(Option<Vec<ConnectPeerAssociation>>)`](crate::output::GetConnectPeerAssociationsOutput::connect_peer_associations): <p>Displays a list of Connect peer associations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetConnectPeerAssociationsOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<GetConnectPeerAssociationsError>`](crate::error::GetConnectPeerAssociationsError)
    pub fn get_connect_peer_associations(&self) -> crate::client::fluent_builders::GetConnectPeerAssociations {
                                crate::client::fluent_builders::GetConnectPeerAssociations::new(self.handle.clone())
                            }
}

