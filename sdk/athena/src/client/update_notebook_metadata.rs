// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateNotebookMetadata`](crate::client::fluent_builders::UpdateNotebookMetadata) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`notebook_id(impl Into<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::notebook_id) / [`set_notebook_id(Option<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::set_notebook_id): <p>The ID of the notebook to update the metadata for.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::set_client_request_token): <p>A unique case-sensitive string used to ensure the request to create the notebook is idempotent (executes only once).</p> <important>   <p>This token is listed as not required because Amazon Web Services SDKs (for example the Amazon Web Services SDK for Java) auto-generate the token for you. If you are not using the Amazon Web Services SDK or the Amazon Web Services CLI, you must provide this token or the action will fail.</p>  </important>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateNotebookMetadata::set_name): <p>The name to update the notebook to.</p>
                            /// - On success, responds with [`UpdateNotebookMetadataOutput`](crate::output::UpdateNotebookMetadataOutput)
                            /// - On failure, responds with [`SdkError<UpdateNotebookMetadataError>`](crate::error::UpdateNotebookMetadataError)
    pub fn update_notebook_metadata(&self) -> crate::client::fluent_builders::UpdateNotebookMetadata {
                                crate::client::fluent_builders::UpdateNotebookMetadata::new(self.handle.clone())
                            }
}

