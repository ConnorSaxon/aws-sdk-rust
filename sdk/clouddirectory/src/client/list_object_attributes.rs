// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListObjectAttributes`](crate::client::fluent_builders::ListObjectAttributes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListObjectAttributes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl Into<String>)`](crate::client::fluent_builders::ListObjectAttributes::directory_arn) / [`set_directory_arn(Option<String>)`](crate::client::fluent_builders::ListObjectAttributes::set_directory_arn): <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides. For more information, see <code>arns</code>.</p>
    ///   - [`object_reference(ObjectReference)`](crate::client::fluent_builders::ListObjectAttributes::object_reference) / [`set_object_reference(Option<ObjectReference>)`](crate::client::fluent_builders::ListObjectAttributes::set_object_reference): <p>The reference that identifies the object whose attributes will be listed.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListObjectAttributes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListObjectAttributes::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListObjectAttributes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListObjectAttributes::set_max_results): <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    ///   - [`consistency_level(ConsistencyLevel)`](crate::client::fluent_builders::ListObjectAttributes::consistency_level) / [`set_consistency_level(Option<ConsistencyLevel>)`](crate::client::fluent_builders::ListObjectAttributes::set_consistency_level): <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    ///   - [`facet_filter(SchemaFacet)`](crate::client::fluent_builders::ListObjectAttributes::facet_filter) / [`set_facet_filter(Option<SchemaFacet>)`](crate::client::fluent_builders::ListObjectAttributes::set_facet_filter): <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
                            /// - On success, responds with [`ListObjectAttributesOutput`](crate::output::ListObjectAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<AttributeKeyAndValue>>)`](crate::output::ListObjectAttributesOutput::attributes): <p>Attributes map that is associated with the object. <code>AttributeArn</code> is the key, and attribute value is the value.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListObjectAttributesOutput::next_token): <p>The pagination token.</p>
                            /// - On failure, responds with [`SdkError<ListObjectAttributesError>`](crate::error::ListObjectAttributesError)
    pub fn list_object_attributes(&self) -> crate::client::fluent_builders::ListObjectAttributes {
                                crate::client::fluent_builders::ListObjectAttributes::new(self.handle.clone())
                            }
}

