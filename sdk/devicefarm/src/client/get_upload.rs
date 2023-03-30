// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetUpload`](crate::client::fluent_builders::GetUpload) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetUpload::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetUpload::set_arn): <p>The upload's ARN.</p>
                            /// - On success, responds with [`GetUploadOutput`](crate::output::GetUploadOutput) with field(s):
    ///   - [`upload(Option<Upload>)`](crate::output::GetUploadOutput::upload): <p>An app or a set of one or more tests to upload or that have been uploaded.</p>
                            /// - On failure, responds with [`SdkError<GetUploadError>`](crate::error::GetUploadError)
    pub fn get_upload(&self) -> crate::client::fluent_builders::GetUpload {
                                crate::client::fluent_builders::GetUpload::new(self.handle.clone())
                            }
}

