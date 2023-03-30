// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetJobTagging`](crate::client::fluent_builders::GetJobTagging) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetJobTagging::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetJobTagging::set_account_id): <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p>
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::GetJobTagging::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::GetJobTagging::set_job_id): <p>The ID for the S3 Batch Operations job whose tags you want to retrieve.</p>
                            /// - On success, responds with [`GetJobTaggingOutput`](crate::output::GetJobTaggingOutput) with field(s):
    ///   - [`tags(Option<Vec<S3Tag>>)`](crate::output::GetJobTaggingOutput::tags): <p>The set of tags associated with the S3 Batch Operations job.</p>
                            /// - On failure, responds with [`SdkError<GetJobTaggingError>`](crate::error::GetJobTaggingError)
    pub fn get_job_tagging(&self) -> crate::client::fluent_builders::GetJobTagging {
                                crate::client::fluent_builders::GetJobTagging::new(self.handle.clone())
                            }
}

