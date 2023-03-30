// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddListenerCertificates`](crate::client::fluent_builders::AddListenerCertificates) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`listener_arn(impl Into<String>)`](crate::client::fluent_builders::AddListenerCertificates::listener_arn) / [`set_listener_arn(Option<String>)`](crate::client::fluent_builders::AddListenerCertificates::set_listener_arn): <p>The Amazon Resource Name (ARN) of the listener.</p>
    ///   - [`certificates(Vec<Certificate>)`](crate::client::fluent_builders::AddListenerCertificates::certificates) / [`set_certificates(Option<Vec<Certificate>>)`](crate::client::fluent_builders::AddListenerCertificates::set_certificates): <p>The certificate to add. You can specify one certificate per call. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p>
                            /// - On success, responds with [`AddListenerCertificatesOutput`](crate::output::AddListenerCertificatesOutput) with field(s):
    ///   - [`certificates(Option<Vec<Certificate>>)`](crate::output::AddListenerCertificatesOutput::certificates): <p>Information about the certificates in the certificate list.</p>
                            /// - On failure, responds with [`SdkError<AddListenerCertificatesError>`](crate::error::AddListenerCertificatesError)
    pub fn add_listener_certificates(&self) -> crate::client::fluent_builders::AddListenerCertificates {
                                crate::client::fluent_builders::AddListenerCertificates::new(self.handle.clone())
                            }
}

