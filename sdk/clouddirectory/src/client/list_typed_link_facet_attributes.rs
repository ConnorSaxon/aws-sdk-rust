// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTypedLinkFacetAttributes`](crate::client::fluent_builders::ListTypedLinkFacetAttributes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schema_arn(impl Into<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::schema_arn) / [`set_schema_arn(Option<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::set_schema_arn): <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::set_name): <p>The unique name of the typed link facet.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListTypedLinkFacetAttributes::set_max_results): <p>The maximum number of results to retrieve.</p>
                            /// - On success, responds with [`ListTypedLinkFacetAttributesOutput`](crate::output::ListTypedLinkFacetAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<TypedLinkAttributeDefinition>>)`](crate::output::ListTypedLinkFacetAttributesOutput::attributes): <p>An ordered set of attributes associate with the typed link.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTypedLinkFacetAttributesOutput::next_token): <p>The pagination token.</p>
                            /// - On failure, responds with [`SdkError<ListTypedLinkFacetAttributesError>`](crate::error::ListTypedLinkFacetAttributesError)
    pub fn list_typed_link_facet_attributes(&self) -> crate::client::fluent_builders::ListTypedLinkFacetAttributes {
                                crate::client::fluent_builders::ListTypedLinkFacetAttributes::new(self.handle.clone())
                            }
}

