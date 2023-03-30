// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchThings`](crate::client::fluent_builders::SearchThings) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchThings::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`entity_id(impl Into<String>)`](crate::client::fluent_builders::SearchThings::entity_id) / [`set_entity_id(Option<String>)`](crate::client::fluent_builders::SearchThings::set_entity_id): <p>The ID of the entity to which the things are associated.</p>  <p>The IDs should be in the following format.</p>  <p> <code>urn:tdm:REGION/ACCOUNT ID/default:device:DEVICENAME</code> </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchThings::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchThings::set_next_token): <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchThings::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchThings::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`namespace_version(i64)`](crate::client::fluent_builders::SearchThings::namespace_version) / [`set_namespace_version(Option<i64>)`](crate::client::fluent_builders::SearchThings::set_namespace_version): <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
                            /// - On success, responds with [`SearchThingsOutput`](crate::output::SearchThingsOutput) with field(s):
    ///   - [`things(Option<Vec<Thing>>)`](crate::output::SearchThingsOutput::things): <p>An array of things in the result set.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchThingsOutput::next_token): <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
                            /// - On failure, responds with [`SdkError<SearchThingsError>`](crate::error::SearchThingsError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn search_things(&self) -> crate::client::fluent_builders::SearchThings {
                                crate::client::fluent_builders::SearchThings::new(self.handle.clone())
                            }
}

