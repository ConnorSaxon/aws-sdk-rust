// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeCACertificate`](crate::client::fluent_builders::DescribeCACertificate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_id(impl Into<String>)`](crate::client::fluent_builders::DescribeCACertificate::certificate_id) / [`set_certificate_id(Option<String>)`](crate::client::fluent_builders::DescribeCACertificate::set_certificate_id): <p>The CA certificate identifier.</p>
                            /// - On success, responds with [`DescribeCaCertificateOutput`](crate::output::DescribeCaCertificateOutput) with field(s):
    ///   - [`certificate_description(Option<CaCertificateDescription>)`](crate::output::DescribeCaCertificateOutput::certificate_description): <p>The CA certificate description.</p>
    ///   - [`registration_config(Option<RegistrationConfig>)`](crate::output::DescribeCaCertificateOutput::registration_config): <p>Information about the registration configuration.</p>
                            /// - On failure, responds with [`SdkError<DescribeCACertificateError>`](crate::error::DescribeCACertificateError)
    pub fn describe_ca_certificate(&self) -> crate::client::fluent_builders::DescribeCACertificate {
                                crate::client::fluent_builders::DescribeCACertificate::new(self.handle.clone())
                            }
}

