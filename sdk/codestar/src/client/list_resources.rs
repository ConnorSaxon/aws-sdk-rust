// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResources`](crate::client::fluent_builders::ListResources) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_id(impl Into<String>)`](crate::client::fluent_builders::ListResources::project_id) / [`set_project_id(Option<String>)`](crate::client::fluent_builders::ListResources::set_project_id): <p>The ID of the project.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListResources::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListResources::set_next_token): <p>The continuation token for the next set of results, if the results cannot be returned in one response.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListResources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListResources::set_max_results): <p>The maximum amount of data that can be contained in a single set of results.</p>
                            /// - On success, responds with [`ListResourcesOutput`](crate::output::ListResourcesOutput) with field(s):
    ///   - [`resources(Option<Vec<Resource>>)`](crate::output::ListResourcesOutput::resources): <p>An array of resources associated with the project. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListResourcesOutput::next_token): <p>The continuation token to use when requesting the next set of results, if there are more results to be returned.</p>
                            /// - On failure, responds with [`SdkError<ListResourcesError>`](crate::error::ListResourcesError)
    pub fn list_resources(&self) -> crate::client::fluent_builders::ListResources {
                                crate::client::fluent_builders::ListResources::new(self.handle.clone())
                            }
}

