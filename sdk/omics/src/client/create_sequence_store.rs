// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSequenceStore`](crate::client::fluent_builders::CreateSequenceStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateSequenceStore::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateSequenceStore::set_name): <p>A name for the store.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateSequenceStore::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateSequenceStore::set_description): <p>A description for the store.</p>
    ///   - [`sse_config(SseConfig)`](crate::client::fluent_builders::CreateSequenceStore::sse_config) / [`set_sse_config(Option<SseConfig>)`](crate::client::fluent_builders::CreateSequenceStore::set_sse_config): <p>Server-side encryption (SSE) settings for the store.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateSequenceStore::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateSequenceStore::set_tags): <p>Tags for the store.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateSequenceStore::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateSequenceStore::set_client_token): <p>To ensure that requests don't run multiple times, specify a unique token for each request.</p>
                            /// - On success, responds with [`CreateSequenceStoreOutput`](crate::output::CreateSequenceStoreOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::CreateSequenceStoreOutput::id): <p>The store's ID.</p>
    ///   - [`arn(Option<String>)`](crate::output::CreateSequenceStoreOutput::arn): <p>The store's ARN.</p>
    ///   - [`name(Option<String>)`](crate::output::CreateSequenceStoreOutput::name): <p>The store's name.</p>
    ///   - [`description(Option<String>)`](crate::output::CreateSequenceStoreOutput::description): <p>The store's description.</p>
    ///   - [`sse_config(Option<SseConfig>)`](crate::output::CreateSequenceStoreOutput::sse_config): <p>The store's SSE settings.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::CreateSequenceStoreOutput::creation_time): <p>When the store was created.</p>
                            /// - On failure, responds with [`SdkError<CreateSequenceStoreError>`](crate::error::CreateSequenceStoreError)
    pub fn create_sequence_store(&self) -> crate::client::fluent_builders::CreateSequenceStore {
                                crate::client::fluent_builders::CreateSequenceStore::new(self.handle.clone())
                            }
}

