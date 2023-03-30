// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPeerings`](crate::client::fluent_builders::ListPeerings) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPeerings::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`core_network_id(impl Into<String>)`](crate::client::fluent_builders::ListPeerings::core_network_id) / [`set_core_network_id(Option<String>)`](crate::client::fluent_builders::ListPeerings::set_core_network_id): <p>The ID of a core network.</p>
    ///   - [`peering_type(PeeringType)`](crate::client::fluent_builders::ListPeerings::peering_type) / [`set_peering_type(Option<PeeringType>)`](crate::client::fluent_builders::ListPeerings::set_peering_type): <p>Returns a list of a peering requests.</p>
    ///   - [`edge_location(impl Into<String>)`](crate::client::fluent_builders::ListPeerings::edge_location) / [`set_edge_location(Option<String>)`](crate::client::fluent_builders::ListPeerings::set_edge_location): <p>Returns a list edge locations for the </p>
    ///   - [`state(PeeringState)`](crate::client::fluent_builders::ListPeerings::state) / [`set_state(Option<PeeringState>)`](crate::client::fluent_builders::ListPeerings::set_state): <p>Returns a list of the peering request states.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPeerings::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPeerings::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPeerings::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPeerings::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`ListPeeringsOutput`](crate::output::ListPeeringsOutput) with field(s):
    ///   - [`peerings(Option<Vec<Peering>>)`](crate::output::ListPeeringsOutput::peerings): <p>Lists the transit gateway peerings for the <code>ListPeerings</code> request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPeeringsOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListPeeringsError>`](crate::error::ListPeeringsError)
    pub fn list_peerings(&self) -> crate::client::fluent_builders::ListPeerings {
                                crate::client::fluent_builders::ListPeerings::new(self.handle.clone())
                            }
}

