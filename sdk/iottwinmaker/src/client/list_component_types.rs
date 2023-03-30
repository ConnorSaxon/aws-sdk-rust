// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListComponentTypes`](crate::client::fluent_builders::ListComponentTypes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListComponentTypes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::ListComponentTypes::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::ListComponentTypes::set_workspace_id): <p>The ID of the workspace.</p>
    ///   - [`filters(Vec<ListComponentTypesFilter>)`](crate::client::fluent_builders::ListComponentTypes::filters) / [`set_filters(Option<Vec<ListComponentTypesFilter>>)`](crate::client::fluent_builders::ListComponentTypes::set_filters): <p>A list of objects that filter the request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListComponentTypes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListComponentTypes::set_next_token): <p>The string that specifies the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListComponentTypes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListComponentTypes::set_max_results): <p>The maximum number of results to return at one time. The default is 25.</p>  <p>Valid Range: Minimum value of 1. Maximum value of 250.</p>
                            /// - On success, responds with [`ListComponentTypesOutput`](crate::output::ListComponentTypesOutput) with field(s):
    ///   - [`workspace_id(Option<String>)`](crate::output::ListComponentTypesOutput::workspace_id): <p>The ID of the workspace.</p>
    ///   - [`component_type_summaries(Option<Vec<ComponentTypeSummary>>)`](crate::output::ListComponentTypesOutput::component_type_summaries): <p>A list of objects that contain information about the component types.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListComponentTypesOutput::next_token): <p>The string that specifies the next page of results.</p>
    ///   - [`max_results(Option<i32>)`](crate::output::ListComponentTypesOutput::max_results): <p>Specifies the maximum number of results to display.</p>
                            /// - On failure, responds with [`SdkError<ListComponentTypesError>`](crate::error::ListComponentTypesError)
    pub fn list_component_types(&self) -> crate::client::fluent_builders::ListComponentTypes {
                                crate::client::fluent_builders::ListComponentTypes::new(self.handle.clone())
                            }
}

