// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListProjects`](crate::client::fluent_builders::ListProjects) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListProjects::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_id(impl Into<String>)`](crate::client::fluent_builders::ListProjects::portal_id) / [`set_portal_id(Option<String>)`](crate::client::fluent_builders::ListProjects::set_portal_id): <p>The ID of the portal.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListProjects::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListProjects::set_next_token): <p>The token to be used for the next set of paginated results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListProjects::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListProjects::set_max_results): <p>The maximum number of results to return for each paginated request.</p>  <p>Default: 50</p>
                            /// - On success, responds with [`ListProjectsOutput`](crate::output::ListProjectsOutput) with field(s):
    ///   - [`project_summaries(Option<Vec<ProjectSummary>>)`](crate::output::ListProjectsOutput::project_summaries): <p>A list that summarizes each project in the portal.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListProjectsOutput::next_token): <p>The token for the next set of results, or null if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListProjectsError>`](crate::error::ListProjectsError)
    pub fn list_projects(&self) -> crate::client::fluent_builders::ListProjects {
                                crate::client::fluent_builders::ListProjects::new(self.handle.clone())
                            }
}

