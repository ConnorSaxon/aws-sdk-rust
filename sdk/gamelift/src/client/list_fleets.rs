// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFleets`](crate::client::fluent_builders::ListFleets) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFleets::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`build_id(impl Into<String>)`](crate::client::fluent_builders::ListFleets::build_id) / [`set_build_id(Option<String>)`](crate::client::fluent_builders::ListFleets::set_build_id): <p>A unique identifier for the build to request fleets for. Use this parameter to return only fleets using a specified build. Use either the build ID or ARN value.</p>
    ///   - [`script_id(impl Into<String>)`](crate::client::fluent_builders::ListFleets::script_id) / [`set_script_id(Option<String>)`](crate::client::fluent_builders::ListFleets::set_script_id): <p>A unique identifier for the Realtime script to request fleets for. Use this parameter to return only fleets using a specified script. Use either the script ID or ARN value.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListFleets::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListFleets::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFleets::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFleets::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
                            /// - On success, responds with [`ListFleetsOutput`](crate::output::ListFleetsOutput) with field(s):
    ///   - [`fleet_ids(Option<Vec<String>>)`](crate::output::ListFleetsOutput::fleet_ids): <p>A set of fleet IDs that match the list request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListFleetsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
                            /// - On failure, responds with [`SdkError<ListFleetsError>`](crate::error::ListFleetsError)
    pub fn list_fleets(&self) -> crate::client::fluent_builders::ListFleets {
                                crate::client::fluent_builders::ListFleets::new(self.handle.clone())
                            }
}

