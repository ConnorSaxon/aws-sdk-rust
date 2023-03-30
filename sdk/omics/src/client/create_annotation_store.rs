// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAnnotationStore`](crate::client::fluent_builders::CreateAnnotationStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`reference(ReferenceItem)`](crate::client::fluent_builders::CreateAnnotationStore::reference) / [`set_reference(Option<ReferenceItem>)`](crate::client::fluent_builders::CreateAnnotationStore::set_reference): <p>The genome reference for the store's annotations.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateAnnotationStore::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateAnnotationStore::set_name): <p>A name for the store.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateAnnotationStore::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateAnnotationStore::set_description): <p>A description for the store.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateAnnotationStore::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateAnnotationStore::set_tags): <p>Tags for the store.</p>
    ///   - [`sse_config(SseConfig)`](crate::client::fluent_builders::CreateAnnotationStore::sse_config) / [`set_sse_config(Option<SseConfig>)`](crate::client::fluent_builders::CreateAnnotationStore::set_sse_config): <p>Server-side encryption (SSE) settings for the store.</p>
    ///   - [`store_format(StoreFormat)`](crate::client::fluent_builders::CreateAnnotationStore::store_format) / [`set_store_format(Option<StoreFormat>)`](crate::client::fluent_builders::CreateAnnotationStore::set_store_format): <p>The annotation file format of the store.</p>
    ///   - [`store_options(StoreOptions)`](crate::client::fluent_builders::CreateAnnotationStore::store_options) / [`set_store_options(Option<StoreOptions>)`](crate::client::fluent_builders::CreateAnnotationStore::set_store_options): <p>File parsing options for the annotation store.</p>
                            /// - On success, responds with [`CreateAnnotationStoreOutput`](crate::output::CreateAnnotationStoreOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::CreateAnnotationStoreOutput::id): <p>The store's ID.</p>
    ///   - [`reference(Option<ReferenceItem>)`](crate::output::CreateAnnotationStoreOutput::reference): <p>The store's genome reference.</p>
    ///   - [`store_format(Option<StoreFormat>)`](crate::output::CreateAnnotationStoreOutput::store_format): <p>The annotation file format of the store.</p>
    ///   - [`store_options(Option<StoreOptions>)`](crate::output::CreateAnnotationStoreOutput::store_options): <p>The store's file parsing options.</p>
    ///   - [`status(Option<StoreStatus>)`](crate::output::CreateAnnotationStoreOutput::status): <p>The store's status.</p>
    ///   - [`name(Option<String>)`](crate::output::CreateAnnotationStoreOutput::name): <p>The store's name.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::CreateAnnotationStoreOutput::creation_time): <p>When the store was created.</p>
                            /// - On failure, responds with [`SdkError<CreateAnnotationStoreError>`](crate::error::CreateAnnotationStoreError)
    pub fn create_annotation_store(&self) -> crate::client::fluent_builders::CreateAnnotationStore {
                                crate::client::fluent_builders::CreateAnnotationStore::new(self.handle.clone())
                            }
}

