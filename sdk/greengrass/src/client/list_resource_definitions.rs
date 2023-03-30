// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResourceDefinitions`](crate::client::fluent_builders::ListResourceDefinitions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(impl Into<String>)`](crate::client::fluent_builders::ListResourceDefinitions::max_results) / [`set_max_results(Option<String>)`](crate::client::fluent_builders::ListResourceDefinitions::set_max_results): The maximum number of results to be returned per request.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListResourceDefinitions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListResourceDefinitions::set_next_token): The token for the next set of results, or ''null'' if there are no additional results.
                            /// - On success, responds with [`ListResourceDefinitionsOutput`](crate::output::ListResourceDefinitionsOutput) with field(s):
    ///   - [`definitions(Option<Vec<DefinitionInformation>>)`](crate::output::ListResourceDefinitionsOutput::definitions): Information about a definition.
    ///   - [`next_token(Option<String>)`](crate::output::ListResourceDefinitionsOutput::next_token): The token for the next set of results, or ''null'' if there are no additional results.
                            /// - On failure, responds with [`SdkError<ListResourceDefinitionsError>`](crate::error::ListResourceDefinitionsError)
    pub fn list_resource_definitions(&self) -> crate::client::fluent_builders::ListResourceDefinitions {
                                crate::client::fluent_builders::ListResourceDefinitions::new(self.handle.clone())
                            }
}

