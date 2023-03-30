// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDocument`](crate::client::fluent_builders::DeleteDocument) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::client::fluent_builders::DeleteDocument::authentication_token) / [`set_authentication_token(Option<String>)`](crate::client::fluent_builders::DeleteDocument::set_authentication_token): <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    ///   - [`document_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDocument::document_id) / [`set_document_id(Option<String>)`](crate::client::fluent_builders::DeleteDocument::set_document_id): <p>The ID of the document.</p>
                            /// - On success, responds with [`DeleteDocumentOutput`](crate::output::DeleteDocumentOutput)
                            /// - On failure, responds with [`SdkError<DeleteDocumentError>`](crate::error::DeleteDocumentError)
    pub fn delete_document(&self) -> crate::client::fluent_builders::DeleteDocument {
                                crate::client::fluent_builders::DeleteDocument::new(self.handle.clone())
                            }
}

