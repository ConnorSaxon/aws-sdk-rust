// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetDocumentStatus`](crate::client::fluent_builders::BatchGetDocumentStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::BatchGetDocumentStatus::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::BatchGetDocumentStatus::set_index_id): <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    ///   - [`document_info_list(Vec<DocumentInfo>)`](crate::client::fluent_builders::BatchGetDocumentStatus::document_info_list) / [`set_document_info_list(Option<Vec<DocumentInfo>>)`](crate::client::fluent_builders::BatchGetDocumentStatus::set_document_info_list): <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
                            /// - On success, responds with [`BatchGetDocumentStatusOutput`](crate::output::BatchGetDocumentStatusOutput) with field(s):
    ///   - [`errors(Option<Vec<BatchGetDocumentStatusResponseError>>)`](crate::output::BatchGetDocumentStatusOutput::errors): <p>A list of documents that Amazon Kendra couldn't get the status for. The list includes the ID of the document and the reason that the status couldn't be found.</p>
    ///   - [`document_status_list(Option<Vec<Status>>)`](crate::output::BatchGetDocumentStatusOutput::document_status_list): <p>The status of documents. The status indicates if the document is waiting to be indexed, is in the process of indexing, has completed indexing, or failed indexing. If a document failed indexing, the status provides the reason why.</p>
                            /// - On failure, responds with [`SdkError<BatchGetDocumentStatusError>`](crate::error::BatchGetDocumentStatusError)
    pub fn batch_get_document_status(&self) -> crate::client::fluent_builders::BatchGetDocumentStatus {
                                crate::client::fluent_builders::BatchGetDocumentStatus::new(self.handle.clone())
                            }
}

