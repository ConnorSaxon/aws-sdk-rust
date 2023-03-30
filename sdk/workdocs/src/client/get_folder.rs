// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetFolder`](crate::client::fluent_builders::GetFolder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::client::fluent_builders::GetFolder::authentication_token) / [`set_authentication_token(Option<String>)`](crate::client::fluent_builders::GetFolder::set_authentication_token): <p>Amazon WorkDocs authentication token. Not required when using AWS administrator credentials to access the API.</p>
    ///   - [`folder_id(impl Into<String>)`](crate::client::fluent_builders::GetFolder::folder_id) / [`set_folder_id(Option<String>)`](crate::client::fluent_builders::GetFolder::set_folder_id): <p>The ID of the folder.</p>
    ///   - [`include_custom_metadata(bool)`](crate::client::fluent_builders::GetFolder::include_custom_metadata) / [`set_include_custom_metadata(bool)`](crate::client::fluent_builders::GetFolder::set_include_custom_metadata): <p>Set to TRUE to include custom metadata in the response.</p>
                            /// - On success, responds with [`GetFolderOutput`](crate::output::GetFolderOutput) with field(s):
    ///   - [`metadata(Option<FolderMetadata>)`](crate::output::GetFolderOutput::metadata): <p>The metadata of the folder.</p>
    ///   - [`custom_metadata(Option<HashMap<String, String>>)`](crate::output::GetFolderOutput::custom_metadata): <p>The custom metadata on the folder.</p>
                            /// - On failure, responds with [`SdkError<GetFolderError>`](crate::error::GetFolderError)
    pub fn get_folder(&self) -> crate::client::fluent_builders::GetFolder {
                                crate::client::fluent_builders::GetFolder::new(self.handle.clone())
                            }
}

