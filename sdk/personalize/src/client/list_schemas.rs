// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSchemas`](crate::client::fluent_builders::ListSchemas) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSchemas::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_next_token): <p>A token returned from the previous call to <code>ListSchemas</code> for getting the next set of schemas (if they exist).</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSchemas::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListSchemas::set_max_results): <p>The maximum number of schemas to return.</p>
                            /// - On success, responds with [`ListSchemasOutput`](crate::output::ListSchemasOutput) with field(s):
    ///   - [`schemas(Option<Vec<DatasetSchemaSummary>>)`](crate::output::ListSchemasOutput::schemas): <p>A list of schemas.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSchemasOutput::next_token): <p>A token used to get the next set of schemas (if they exist).</p>
                            /// - On failure, responds with [`SdkError<ListSchemasError>`](crate::error::ListSchemasError)
    pub fn list_schemas(&self) -> crate::client::fluent_builders::ListSchemas {
                                crate::client::fluent_builders::ListSchemas::new(self.handle.clone())
                            }
}

