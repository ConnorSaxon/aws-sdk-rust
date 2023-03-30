// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RevokeSignature`](crate::client::fluent_builders::RevokeSignature) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::RevokeSignature::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::RevokeSignature::set_job_id): <p>ID of the signing job to be revoked.</p>
    ///   - [`job_owner(impl Into<String>)`](crate::client::fluent_builders::RevokeSignature::job_owner) / [`set_job_owner(Option<String>)`](crate::client::fluent_builders::RevokeSignature::set_job_owner): <p>AWS account ID of the job owner.</p>
    ///   - [`reason(impl Into<String>)`](crate::client::fluent_builders::RevokeSignature::reason) / [`set_reason(Option<String>)`](crate::client::fluent_builders::RevokeSignature::set_reason): <p>The reason for revoking the signing job.</p>
                            /// - On success, responds with [`RevokeSignatureOutput`](crate::output::RevokeSignatureOutput)
                            /// - On failure, responds with [`SdkError<RevokeSignatureError>`](crate::error::RevokeSignatureError)
    pub fn revoke_signature(&self) -> crate::client::fluent_builders::RevokeSignature {
                                crate::client::fluent_builders::RevokeSignature::new(self.handle.clone())
                            }
}

