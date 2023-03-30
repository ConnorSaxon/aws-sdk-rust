// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchEntities`](crate::client::fluent_builders::SearchEntities) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchEntities::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`entity_types(Vec<EntityType>)`](crate::client::fluent_builders::SearchEntities::entity_types) / [`set_entity_types(Option<Vec<EntityType>>)`](crate::client::fluent_builders::SearchEntities::set_entity_types): <p>The entity types for which to search.</p>
    ///   - [`filters(Vec<EntityFilter>)`](crate::client::fluent_builders::SearchEntities::filters) / [`set_filters(Option<Vec<EntityFilter>>)`](crate::client::fluent_builders::SearchEntities::set_filters): <p>Optional filter to apply to the search. Valid filters are <code>NAME</code> <code>NAMESPACE</code>, <code>SEMANTIC_TYPE_PATH</code> and <code>REFERENCED_ENTITY_ID</code>. <code>REFERENCED_ENTITY_ID</code> filters on entities that are used by the entity in the result set. For example, you can filter on the ID of a property that is used in a state.</p>  <p>Multiple filters function as OR criteria in the query. Multiple values passed inside the filter function as AND criteria.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchEntities::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchEntities::set_next_token): <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchEntities::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchEntities::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`namespace_version(i64)`](crate::client::fluent_builders::SearchEntities::namespace_version) / [`set_namespace_version(Option<i64>)`](crate::client::fluent_builders::SearchEntities::set_namespace_version): <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
                            /// - On success, responds with [`SearchEntitiesOutput`](crate::output::SearchEntitiesOutput) with field(s):
    ///   - [`descriptions(Option<Vec<EntityDescription>>)`](crate::output::SearchEntitiesOutput::descriptions): <p>An array of descriptions for each entity returned in the search result.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchEntitiesOutput::next_token): <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
                            /// - On failure, responds with [`SdkError<SearchEntitiesError>`](crate::error::SearchEntitiesError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn search_entities(&self) -> crate::client::fluent_builders::SearchEntities {
                                crate::client::fluent_builders::SearchEntities::new(self.handle.clone())
                            }
}

