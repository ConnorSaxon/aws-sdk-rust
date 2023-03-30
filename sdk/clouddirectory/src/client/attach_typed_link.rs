// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AttachTypedLink`](crate::client::fluent_builders::AttachTypedLink) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl Into<String>)`](crate::client::fluent_builders::AttachTypedLink::directory_arn) / [`set_directory_arn(Option<String>)`](crate::client::fluent_builders::AttachTypedLink::set_directory_arn): <p>The Amazon Resource Name (ARN) of the directory where you want to attach the typed link.</p>
    ///   - [`source_object_reference(ObjectReference)`](crate::client::fluent_builders::AttachTypedLink::source_object_reference) / [`set_source_object_reference(Option<ObjectReference>)`](crate::client::fluent_builders::AttachTypedLink::set_source_object_reference): <p>Identifies the source object that the typed link will attach to.</p>
    ///   - [`target_object_reference(ObjectReference)`](crate::client::fluent_builders::AttachTypedLink::target_object_reference) / [`set_target_object_reference(Option<ObjectReference>)`](crate::client::fluent_builders::AttachTypedLink::set_target_object_reference): <p>Identifies the target object that the typed link will attach to.</p>
    ///   - [`typed_link_facet(TypedLinkSchemaAndFacetName)`](crate::client::fluent_builders::AttachTypedLink::typed_link_facet) / [`set_typed_link_facet(Option<TypedLinkSchemaAndFacetName>)`](crate::client::fluent_builders::AttachTypedLink::set_typed_link_facet): <p>Identifies the typed link facet that is associated with the typed link.</p>
    ///   - [`attributes(Vec<AttributeNameAndValue>)`](crate::client::fluent_builders::AttachTypedLink::attributes) / [`set_attributes(Option<Vec<AttributeNameAndValue>>)`](crate::client::fluent_builders::AttachTypedLink::set_attributes): <p>A set of attributes that are associated with the typed link.</p>
                            /// - On success, responds with [`AttachTypedLinkOutput`](crate::output::AttachTypedLinkOutput) with field(s):
    ///   - [`typed_link_specifier(Option<TypedLinkSpecifier>)`](crate::output::AttachTypedLinkOutput::typed_link_specifier): <p>Returns a typed link specifier as output.</p>
                            /// - On failure, responds with [`SdkError<AttachTypedLinkError>`](crate::error::AttachTypedLinkError)
    pub fn attach_typed_link(&self) -> crate::client::fluent_builders::AttachTypedLink {
                                crate::client::fluent_builders::AttachTypedLink::new(self.handle.clone())
                            }
}

