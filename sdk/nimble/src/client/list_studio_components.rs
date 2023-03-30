// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListStudioComponents`](crate::client::fluent_builders::ListStudioComponents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListStudioComponents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListStudioComponents::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListStudioComponents::set_max_results): <p>The max number of results to return in the response.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListStudioComponents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListStudioComponents::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`states(Vec<StudioComponentState>)`](crate::client::fluent_builders::ListStudioComponents::states) / [`set_states(Option<Vec<StudioComponentState>>)`](crate::client::fluent_builders::ListStudioComponents::set_states): <p>Filters the request to studio components that are in one of the given states. </p>
    ///   - [`studio_id(impl Into<String>)`](crate::client::fluent_builders::ListStudioComponents::studio_id) / [`set_studio_id(Option<String>)`](crate::client::fluent_builders::ListStudioComponents::set_studio_id): <p>The studio ID. </p>
    ///   - [`types(Vec<StudioComponentType>)`](crate::client::fluent_builders::ListStudioComponents::types) / [`set_types(Option<Vec<StudioComponentType>>)`](crate::client::fluent_builders::ListStudioComponents::set_types): <p>Filters the request to studio components that are of one of the given types.</p>
                            /// - On success, responds with [`ListStudioComponentsOutput`](crate::output::ListStudioComponentsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListStudioComponentsOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`studio_components(Option<Vec<StudioComponent>>)`](crate::output::ListStudioComponentsOutput::studio_components): <p>A collection of studio components.</p>
                            /// - On failure, responds with [`SdkError<ListStudioComponentsError>`](crate::error::ListStudioComponentsError)
    pub fn list_studio_components(&self) -> crate::client::fluent_builders::ListStudioComponents {
                                crate::client::fluent_builders::ListStudioComponents::new(self.handle.clone())
                            }
}

