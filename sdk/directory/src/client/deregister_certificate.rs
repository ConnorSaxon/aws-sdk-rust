// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterCertificate`](crate::client::fluent_builders::DeregisterCertificate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterCertificate::directory_id) / [`set_directory_id(Option<String>)`](crate::client::fluent_builders::DeregisterCertificate::set_directory_id): <p>The identifier of the directory.</p>
    ///   - [`certificate_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterCertificate::certificate_id) / [`set_certificate_id(Option<String>)`](crate::client::fluent_builders::DeregisterCertificate::set_certificate_id): <p>The identifier of the certificate.</p>
                            /// - On success, responds with [`DeregisterCertificateOutput`](crate::output::DeregisterCertificateOutput)
                            /// - On failure, responds with [`SdkError<DeregisterCertificateError>`](crate::error::DeregisterCertificateError)
    pub fn deregister_certificate(&self) -> crate::client::fluent_builders::DeregisterCertificate {
                                crate::client::fluent_builders::DeregisterCertificate::new(self.handle.clone())
                            }
}

