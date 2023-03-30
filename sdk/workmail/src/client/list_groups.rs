// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListGroups`](crate::client::fluent_builders::ListGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListGroups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::ListGroups::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::ListGroups::set_organization_id): <p>The identifier for the organization under which the groups exist.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListGroups::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListGroups::set_next_token): <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListGroups::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListGroups::set_max_results): <p>The maximum number of results to return in a single call.</p>
                            /// - On success, responds with [`ListGroupsOutput`](crate::output::ListGroupsOutput) with field(s):
    ///   - [`groups(Option<Vec<Group>>)`](crate::output::ListGroupsOutput::groups): <p>The overview of groups for an organization.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListGroupsOutput::next_token): <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListGroupsError>`](crate::error::ListGroupsError)
    pub fn list_groups(&self) -> crate::client::fluent_builders::ListGroups {
                                crate::client::fluent_builders::ListGroups::new(self.handle.clone())
                            }
}

