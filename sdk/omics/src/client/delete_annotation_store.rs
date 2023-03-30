// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAnnotationStore`](crate::client::fluent_builders::DeleteAnnotationStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteAnnotationStore::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteAnnotationStore::set_name): <p>The store's name.</p>
    ///   - [`force(bool)`](crate::client::fluent_builders::DeleteAnnotationStore::force) / [`set_force(Option<bool>)`](crate::client::fluent_builders::DeleteAnnotationStore::set_force): <p>Whether to force deletion.</p>
                            /// - On success, responds with [`DeleteAnnotationStoreOutput`](crate::output::DeleteAnnotationStoreOutput) with field(s):
    ///   - [`status(Option<StoreStatus>)`](crate::output::DeleteAnnotationStoreOutput::status): <p>The store's status.</p>
                            /// - On failure, responds with [`SdkError<DeleteAnnotationStoreError>`](crate::error::DeleteAnnotationStoreError)
    pub fn delete_annotation_store(&self) -> crate::client::fluent_builders::DeleteAnnotationStore {
                                crate::client::fluent_builders::DeleteAnnotationStore::new(self.handle.clone())
                            }
}

