// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListComponents`](crate::client::fluent_builders::ListComponents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListComponents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::ListComponents::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::ListComponents::set_app_id): <p>The unique ID for the Amplify app.</p>
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::ListComponents::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::ListComponents::set_environment_name): <p>The name of the backend environment that is a part of the Amplify app.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListComponents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListComponents::set_next_token): <p>The token to request the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListComponents::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListComponents::set_max_results): <p>The maximum number of components to retrieve.</p>
                            /// - On success, responds with [`ListComponentsOutput`](crate::output::ListComponentsOutput) with field(s):
    ///   - [`entities(Option<Vec<ComponentSummary>>)`](crate::output::ListComponentsOutput::entities): <p>The list of components for the Amplify app.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListComponentsOutput::next_token): <p>The pagination token that's included if more results are available.</p>
                            /// - On failure, responds with [`SdkError<ListComponentsError>`](crate::error::ListComponentsError)
    pub fn list_components(&self) -> crate::client::fluent_builders::ListComponents {
                                crate::client::fluent_builders::ListComponents::new(self.handle.clone())
                            }
}

