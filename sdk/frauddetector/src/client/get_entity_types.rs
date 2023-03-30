// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEntityTypes`](crate::client::fluent_builders::GetEntityTypes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetEntityTypes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetEntityTypes::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetEntityTypes::set_name): <p>The name.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetEntityTypes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetEntityTypes::set_next_token): <p>The next token for the subsequent request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetEntityTypes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetEntityTypes::set_max_results): <p>The maximum number of objects to return for the request.</p>
                            /// - On success, responds with [`GetEntityTypesOutput`](crate::output::GetEntityTypesOutput) with field(s):
    ///   - [`entity_types(Option<Vec<EntityType>>)`](crate::output::GetEntityTypesOutput::entity_types): <p>An array of entity types.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetEntityTypesOutput::next_token): <p>The next page token.</p>
                            /// - On failure, responds with [`SdkError<GetEntityTypesError>`](crate::error::GetEntityTypesError)
    pub fn get_entity_types(&self) -> crate::client::fluent_builders::GetEntityTypes {
                                crate::client::fluent_builders::GetEntityTypes::new(self.handle.clone())
                            }
}

