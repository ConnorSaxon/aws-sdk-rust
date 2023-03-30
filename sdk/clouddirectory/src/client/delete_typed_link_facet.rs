// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTypedLinkFacet`](crate::client::fluent_builders::DeleteTypedLinkFacet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schema_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteTypedLinkFacet::schema_arn) / [`set_schema_arn(Option<String>)`](crate::client::fluent_builders::DeleteTypedLinkFacet::set_schema_arn): <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteTypedLinkFacet::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteTypedLinkFacet::set_name): <p>The unique name of the typed link facet.</p>
                            /// - On success, responds with [`DeleteTypedLinkFacetOutput`](crate::output::DeleteTypedLinkFacetOutput)
                            /// - On failure, responds with [`SdkError<DeleteTypedLinkFacetError>`](crate::error::DeleteTypedLinkFacetError)
    pub fn delete_typed_link_facet(&self) -> crate::client::fluent_builders::DeleteTypedLinkFacet {
                                crate::client::fluent_builders::DeleteTypedLinkFacet::new(self.handle.clone())
                            }
}

