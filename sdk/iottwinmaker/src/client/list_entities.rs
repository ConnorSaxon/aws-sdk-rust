// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEntities`](crate::client::fluent_builders::ListEntities) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEntities::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::ListEntities::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::ListEntities::set_workspace_id): <p>The ID of the workspace.</p>
    ///   - [`filters(Vec<ListEntitiesFilter>)`](crate::client::fluent_builders::ListEntities::filters) / [`set_filters(Option<Vec<ListEntitiesFilter>>)`](crate::client::fluent_builders::ListEntities::set_filters): <p>A list of objects that filter the request.</p> <note>   <p>Only one object is accepted as a valid input.</p>  </note>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEntities::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListEntities::set_max_results): <p>The maximum number of results to return at one time. The default is 25.</p>  <p>Valid Range: Minimum value of 1. Maximum value of 250.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEntities::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEntities::set_next_token): <p>The string that specifies the next page of results.</p>
                            /// - On success, responds with [`ListEntitiesOutput`](crate::output::ListEntitiesOutput) with field(s):
    ///   - [`entity_summaries(Option<Vec<EntitySummary>>)`](crate::output::ListEntitiesOutput::entity_summaries): <p>A list of objects that contain information about the entities.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListEntitiesOutput::next_token): <p>The string that specifies the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListEntitiesError>`](crate::error::ListEntitiesError)
    pub fn list_entities(&self) -> crate::client::fluent_builders::ListEntities {
                                crate::client::fluent_builders::ListEntities::new(self.handle.clone())
                            }
}

