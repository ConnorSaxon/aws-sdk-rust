// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEbsDefaultKmsKeyId`](crate::client::fluent_builders::GetEbsDefaultKmsKeyId) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::GetEbsDefaultKmsKeyId::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::GetEbsDefaultKmsKeyId::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`GetEbsDefaultKmsKeyIdOutput`](crate::output::GetEbsDefaultKmsKeyIdOutput) with field(s):
    ///   - [`kms_key_id(Option<String>)`](crate::output::GetEbsDefaultKmsKeyIdOutput::kms_key_id): <p>The Amazon Resource Name (ARN) of the default KMS key for encryption by default.</p>
                            /// - On failure, responds with [`SdkError<GetEbsDefaultKmsKeyIdError>`](crate::error::GetEbsDefaultKmsKeyIdError)
    pub fn get_ebs_default_kms_key_id(&self) -> crate::client::fluent_builders::GetEbsDefaultKmsKeyId {
                                crate::client::fluent_builders::GetEbsDefaultKmsKeyId::new(self.handle.clone())
                            }
}

