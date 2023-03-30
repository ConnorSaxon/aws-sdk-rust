// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RevokeCertificate`](crate::client::fluent_builders::RevokeCertificate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_authority_arn(impl Into<String>)`](crate::client::fluent_builders::RevokeCertificate::certificate_authority_arn) / [`set_certificate_authority_arn(Option<String>)`](crate::client::fluent_builders::RevokeCertificate::set_certificate_authority_arn): <p>Amazon Resource Name (ARN) of the private CA that issued the certificate to be revoked. This must be of the form:</p>  <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    ///   - [`certificate_serial(impl Into<String>)`](crate::client::fluent_builders::RevokeCertificate::certificate_serial) / [`set_certificate_serial(Option<String>)`](crate::client::fluent_builders::RevokeCertificate::set_certificate_serial): <p>Serial number of the certificate to be revoked. This must be in hexadecimal format. You can retrieve the serial number by calling <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_GetCertificate.html">GetCertificate</a> with the Amazon Resource Name (ARN) of the certificate you want and the ARN of your private CA. The <b>GetCertificate</b> action retrieves the certificate in the PEM format. You can use the following OpenSSL command to list the certificate in text format and copy the hexadecimal serial number. </p>  <p> <code>openssl x509 -in <i>file_path</i> -text -noout</code> </p>  <p>You can also copy the serial number from the console or use the <a href="https://docs.aws.amazon.com/acm/latest/APIReference/API_DescribeCertificate.html">DescribeCertificate</a> action in the <i>Certificate Manager API Reference</i>. </p>
    ///   - [`revocation_reason(RevocationReason)`](crate::client::fluent_builders::RevokeCertificate::revocation_reason) / [`set_revocation_reason(Option<RevocationReason>)`](crate::client::fluent_builders::RevokeCertificate::set_revocation_reason): <p>Specifies why you revoked the certificate.</p>
                            /// - On success, responds with [`RevokeCertificateOutput`](crate::output::RevokeCertificateOutput)
                            /// - On failure, responds with [`SdkError<RevokeCertificateError>`](crate::error::RevokeCertificateError)
    pub fn revoke_certificate(&self) -> crate::client::fluent_builders::RevokeCertificate {
                                crate::client::fluent_builders::RevokeCertificate::new(self.handle.clone())
                            }
}

