// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddFacetToObject`](crate::client::fluent_builders::AddFacetToObject) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl Into<String>)`](crate::client::fluent_builders::AddFacetToObject::directory_arn) / [`set_directory_arn(Option<String>)`](crate::client::fluent_builders::AddFacetToObject::set_directory_arn): <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides. For more information, see <code>arns</code>.</p>
    ///   - [`schema_facet(SchemaFacet)`](crate::client::fluent_builders::AddFacetToObject::schema_facet) / [`set_schema_facet(Option<SchemaFacet>)`](crate::client::fluent_builders::AddFacetToObject::set_schema_facet): <p>Identifiers for the facet that you are adding to the object. See <code>SchemaFacet</code> for details.</p>
    ///   - [`object_attribute_list(Vec<AttributeKeyAndValue>)`](crate::client::fluent_builders::AddFacetToObject::object_attribute_list) / [`set_object_attribute_list(Option<Vec<AttributeKeyAndValue>>)`](crate::client::fluent_builders::AddFacetToObject::set_object_attribute_list): <p>Attributes on the facet that you are adding to the object.</p>
    ///   - [`object_reference(ObjectReference)`](crate::client::fluent_builders::AddFacetToObject::object_reference) / [`set_object_reference(Option<ObjectReference>)`](crate::client::fluent_builders::AddFacetToObject::set_object_reference): <p>A reference to the object you are adding the specified facet to.</p>
                            /// - On success, responds with [`AddFacetToObjectOutput`](crate::output::AddFacetToObjectOutput)
                            /// - On failure, responds with [`SdkError<AddFacetToObjectError>`](crate::error::AddFacetToObjectError)
    pub fn add_facet_to_object(&self) -> crate::client::fluent_builders::AddFacetToObject {
                                crate::client::fluent_builders::AddFacetToObject::new(self.handle.clone())
                            }
}

