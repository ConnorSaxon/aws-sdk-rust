// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLinkAssociations`](crate::client::fluent_builders::GetLinkAssociations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetLinkAssociations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::GetLinkAssociations::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::GetLinkAssociations::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::GetLinkAssociations::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::GetLinkAssociations::set_device_id): <p>The ID of the device.</p>
    ///   - [`link_id(impl Into<String>)`](crate::client::fluent_builders::GetLinkAssociations::link_id) / [`set_link_id(Option<String>)`](crate::client::fluent_builders::GetLinkAssociations::set_link_id): <p>The ID of the link.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetLinkAssociations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetLinkAssociations::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetLinkAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetLinkAssociations::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`GetLinkAssociationsOutput`](crate::output::GetLinkAssociationsOutput) with field(s):
    ///   - [`link_associations(Option<Vec<LinkAssociation>>)`](crate::output::GetLinkAssociationsOutput::link_associations): <p>The link associations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetLinkAssociationsOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<GetLinkAssociationsError>`](crate::error::GetLinkAssociationsError)
    pub fn get_link_associations(&self) -> crate::client::fluent_builders::GetLinkAssociations {
                                crate::client::fluent_builders::GetLinkAssociations::new(self.handle.clone())
                            }
}

