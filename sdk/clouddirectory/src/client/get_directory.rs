// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDirectory`](crate::client::fluent_builders::GetDirectory) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl Into<String>)`](crate::client::fluent_builders::GetDirectory::directory_arn) / [`set_directory_arn(Option<String>)`](crate::client::fluent_builders::GetDirectory::set_directory_arn): <p>The ARN of the directory.</p>
                            /// - On success, responds with [`GetDirectoryOutput`](crate::output::GetDirectoryOutput) with field(s):
    ///   - [`directory(Option<Directory>)`](crate::output::GetDirectoryOutput::directory): <p>Metadata about the directory.</p>
                            /// - On failure, responds with [`SdkError<GetDirectoryError>`](crate::error::GetDirectoryError)
    pub fn get_directory(&self) -> crate::client::fluent_builders::GetDirectory {
                                crate::client::fluent_builders::GetDirectory::new(self.handle.clone())
                            }
}

