// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEnvironments`](crate::client::fluent_builders::ListEnvironments) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEnvironments::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEnvironments::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEnvironments::set_next_token): <p>A token that indicates the location of the next environment in the array of environments, after the list of environments that was previously requested.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEnvironments::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListEnvironments::set_max_results): <p>The maximum number of environments to list.</p>
    ///   - [`environment_templates(Vec<EnvironmentTemplateFilter>)`](crate::client::fluent_builders::ListEnvironments::environment_templates) / [`set_environment_templates(Option<Vec<EnvironmentTemplateFilter>>)`](crate::client::fluent_builders::ListEnvironments::set_environment_templates): <p>An array of the versions of the environment template.</p>
                            /// - On success, responds with [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListEnvironmentsOutput::next_token): <p>A token that indicates the location of the next environment in the array of environments, after the current requested list of environments.</p>
    ///   - [`environments(Option<Vec<EnvironmentSummary>>)`](crate::output::ListEnvironmentsOutput::environments): <p>An array of environment detail data summaries.</p>
                            /// - On failure, responds with [`SdkError<ListEnvironmentsError>`](crate::error::ListEnvironmentsError)
    pub fn list_environments(&self) -> crate::client::fluent_builders::ListEnvironments {
                                crate::client::fluent_builders::ListEnvironments::new(self.handle.clone())
                            }
}

