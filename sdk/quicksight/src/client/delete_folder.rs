// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFolder`](crate::client::fluent_builders::DeleteFolder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteFolder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::DeleteFolder::set_aws_account_id): <p>The ID for the Amazon Web Services account that contains the folder.</p>
    ///   - [`folder_id(impl Into<String>)`](crate::client::fluent_builders::DeleteFolder::folder_id) / [`set_folder_id(Option<String>)`](crate::client::fluent_builders::DeleteFolder::set_folder_id): <p>The ID of the folder.</p>
                            /// - On success, responds with [`DeleteFolderOutput`](crate::output::DeleteFolderOutput) with field(s):
    ///   - [`status(i32)`](crate::output::DeleteFolderOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`arn(Option<String>)`](crate::output::DeleteFolderOutput::arn): <p>The Amazon Resource Name of the deleted folder.</p>
    ///   - [`folder_id(Option<String>)`](crate::output::DeleteFolderOutput::folder_id): <p>The ID of the folder.</p>
    ///   - [`request_id(Option<String>)`](crate::output::DeleteFolderOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<DeleteFolderError>`](crate::error::DeleteFolderError)
    pub fn delete_folder(&self) -> crate::client::fluent_builders::DeleteFolder {
                                crate::client::fluent_builders::DeleteFolder::new(self.handle.clone())
                            }
}

