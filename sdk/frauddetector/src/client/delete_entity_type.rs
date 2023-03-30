// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteEntityType`](crate::client::fluent_builders::DeleteEntityType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteEntityType::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteEntityType::set_name): <p>The name of the entity type to delete.</p>
                            /// - On success, responds with [`DeleteEntityTypeOutput`](crate::output::DeleteEntityTypeOutput)
                            /// - On failure, responds with [`SdkError<DeleteEntityTypeError>`](crate::error::DeleteEntityTypeError)
    pub fn delete_entity_type(&self) -> crate::client::fluent_builders::DeleteEntityType {
                                crate::client::fluent_builders::DeleteEntityType::new(self.handle.clone())
                            }
}

