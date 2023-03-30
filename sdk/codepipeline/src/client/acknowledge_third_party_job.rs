// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcknowledgeThirdPartyJob`](crate::client::fluent_builders::AcknowledgeThirdPartyJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::set_job_id): <p>The unique system-generated ID of the job.</p>
    ///   - [`nonce(impl Into<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::nonce) / [`set_nonce(Option<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::set_nonce): <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response to a <code>GetThirdPartyJobDetails</code> request.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::AcknowledgeThirdPartyJob::set_client_token): <p>The clientToken portion of the clientId and clientToken pair used to verify that the calling entity is allowed access to the job and its details.</p>
                            /// - On success, responds with [`AcknowledgeThirdPartyJobOutput`](crate::output::AcknowledgeThirdPartyJobOutput) with field(s):
    ///   - [`status(Option<JobStatus>)`](crate::output::AcknowledgeThirdPartyJobOutput::status): <p>The status information for the third party job, if any.</p>
                            /// - On failure, responds with [`SdkError<AcknowledgeThirdPartyJobError>`](crate::error::AcknowledgeThirdPartyJobError)
    pub fn acknowledge_third_party_job(&self) -> crate::client::fluent_builders::AcknowledgeThirdPartyJob {
                                crate::client::fluent_builders::AcknowledgeThirdPartyJob::new(self.handle.clone())
                            }
}

