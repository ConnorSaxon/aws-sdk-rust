// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TransferCertificate`](crate::client::fluent_builders::TransferCertificate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_id(impl Into<String>)`](crate::client::fluent_builders::TransferCertificate::certificate_id) / [`set_certificate_id(Option<String>)`](crate::client::fluent_builders::TransferCertificate::set_certificate_id): <p>The ID of the certificate. (The last part of the certificate ARN contains the certificate ID.)</p>
    ///   - [`target_aws_account(impl Into<String>)`](crate::client::fluent_builders::TransferCertificate::target_aws_account) / [`set_target_aws_account(Option<String>)`](crate::client::fluent_builders::TransferCertificate::set_target_aws_account): <p>The Amazon Web Services account.</p>
    ///   - [`transfer_message(impl Into<String>)`](crate::client::fluent_builders::TransferCertificate::transfer_message) / [`set_transfer_message(Option<String>)`](crate::client::fluent_builders::TransferCertificate::set_transfer_message): <p>The transfer message.</p>
                            /// - On success, responds with [`TransferCertificateOutput`](crate::output::TransferCertificateOutput) with field(s):
    ///   - [`transferred_certificate_arn(Option<String>)`](crate::output::TransferCertificateOutput::transferred_certificate_arn): <p>The ARN of the certificate.</p>
                            /// - On failure, responds with [`SdkError<TransferCertificateError>`](crate::error::TransferCertificateError)
    pub fn transfer_certificate(&self) -> crate::client::fluent_builders::TransferCertificate {
                                crate::client::fluent_builders::TransferCertificate::new(self.handle.clone())
                            }
}

