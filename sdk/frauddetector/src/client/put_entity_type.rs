// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutEntityType`](crate::client::fluent_builders::PutEntityType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::PutEntityType::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::PutEntityType::set_name): <p>The name of the entity type.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::PutEntityType::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::PutEntityType::set_description): <p>The description.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::PutEntityType::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::PutEntityType::set_tags): <p>A collection of key and value pairs.</p>
                            /// - On success, responds with [`PutEntityTypeOutput`](crate::output::PutEntityTypeOutput)
                            /// - On failure, responds with [`SdkError<PutEntityTypeError>`](crate::error::PutEntityTypeError)
    pub fn put_entity_type(&self) -> crate::client::fluent_builders::PutEntityType {
                                crate::client::fluent_builders::PutEntityType::new(self.handle.clone())
                            }
}

