// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateCertificateOptions`](crate::client::fluent_builders::UpdateCertificateOptions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateCertificateOptions::certificate_arn) / [`set_certificate_arn(Option<String>)`](crate::client::fluent_builders::UpdateCertificateOptions::set_certificate_arn): <p>ARN of the requested certificate to update. This must be of the form:</p>  <p> <code>arn:aws:acm:us-east-1:<i>account</i>:certificate/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    ///   - [`options(CertificateOptions)`](crate::client::fluent_builders::UpdateCertificateOptions::options) / [`set_options(Option<CertificateOptions>)`](crate::client::fluent_builders::UpdateCertificateOptions::set_options): <p>Use to update the options for your certificate. Currently, you can specify whether to add your certificate to a transparency log. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. </p>
                            /// - On success, responds with [`UpdateCertificateOptionsOutput`](crate::output::UpdateCertificateOptionsOutput)
                            /// - On failure, responds with [`SdkError<UpdateCertificateOptionsError>`](crate::error::UpdateCertificateOptionsError)
    pub fn update_certificate_options(&self) -> crate::client::fluent_builders::UpdateCertificateOptions {
                                crate::client::fluent_builders::UpdateCertificateOptions::new(self.handle.clone())
                            }
}

