// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteCustomMetadata`](crate::client::fluent_builders::DeleteCustomMetadata) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::authentication_token) / [`set_authentication_token(Option<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::set_authentication_token): <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::set_resource_id): <p>The ID of the resource, either a document or folder.</p>
    ///   - [`version_id(impl Into<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::version_id) / [`set_version_id(Option<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::set_version_id): <p>The ID of the version, if the custom metadata is being deleted from a document version.</p>
    ///   - [`keys(Vec<String>)`](crate::client::fluent_builders::DeleteCustomMetadata::keys) / [`set_keys(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteCustomMetadata::set_keys): <p>List of properties to remove.</p>
    ///   - [`delete_all(bool)`](crate::client::fluent_builders::DeleteCustomMetadata::delete_all) / [`set_delete_all(bool)`](crate::client::fluent_builders::DeleteCustomMetadata::set_delete_all): <p>Flag to indicate removal of all custom metadata properties from the specified resource.</p>
                            /// - On success, responds with [`DeleteCustomMetadataOutput`](crate::output::DeleteCustomMetadataOutput)
                            /// - On failure, responds with [`SdkError<DeleteCustomMetadataError>`](crate::error::DeleteCustomMetadataError)
    pub fn delete_custom_metadata(&self) -> crate::client::fluent_builders::DeleteCustomMetadata {
                                crate::client::fluent_builders::DeleteCustomMetadata::new(self.handle.clone())
                            }
}

