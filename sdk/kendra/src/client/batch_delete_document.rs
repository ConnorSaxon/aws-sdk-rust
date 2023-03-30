// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDeleteDocument`](crate::client::fluent_builders::BatchDeleteDocument) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteDocument::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::BatchDeleteDocument::set_index_id): <p>The identifier of the index that contains the documents to delete.</p>
    ///   - [`document_id_list(Vec<String>)`](crate::client::fluent_builders::BatchDeleteDocument::document_id_list) / [`set_document_id_list(Option<Vec<String>>)`](crate::client::fluent_builders::BatchDeleteDocument::set_document_id_list): <p>One or more identifiers for documents to delete from the index.</p>
    ///   - [`data_source_sync_job_metric_target(DataSourceSyncJobMetricTarget)`](crate::client::fluent_builders::BatchDeleteDocument::data_source_sync_job_metric_target) / [`set_data_source_sync_job_metric_target(Option<DataSourceSyncJobMetricTarget>)`](crate::client::fluent_builders::BatchDeleteDocument::set_data_source_sync_job_metric_target): <p>Maps a particular data source sync job to a particular data source.</p>
                            /// - On success, responds with [`BatchDeleteDocumentOutput`](crate::output::BatchDeleteDocumentOutput) with field(s):
    ///   - [`failed_documents(Option<Vec<BatchDeleteDocumentResponseFailedDocument>>)`](crate::output::BatchDeleteDocumentOutput::failed_documents): <p>A list of documents that could not be removed from the index. Each entry contains an error message that indicates why the document couldn't be removed from the index.</p>
                            /// - On failure, responds with [`SdkError<BatchDeleteDocumentError>`](crate::error::BatchDeleteDocumentError)
    pub fn batch_delete_document(&self) -> crate::client::fluent_builders::BatchDeleteDocument {
                                crate::client::fluent_builders::BatchDeleteDocument::new(self.handle.clone())
                            }
}

