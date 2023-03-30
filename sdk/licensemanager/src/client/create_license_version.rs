// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateLicenseVersion`](crate::client::fluent_builders::CreateLicenseVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`license_arn(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::license_arn) / [`set_license_arn(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_license_arn): <p>Amazon Resource Name (ARN) of the license.</p>
    ///   - [`license_name(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::license_name) / [`set_license_name(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_license_name): <p>License name.</p>
    ///   - [`product_name(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::product_name) / [`set_product_name(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_product_name): <p>Product name.</p>
    ///   - [`issuer(Issuer)`](crate::client::fluent_builders::CreateLicenseVersion::issuer) / [`set_issuer(Option<Issuer>)`](crate::client::fluent_builders::CreateLicenseVersion::set_issuer): <p>License issuer.</p>
    ///   - [`home_region(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::home_region) / [`set_home_region(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_home_region): <p>Home Region of the license.</p>
    ///   - [`validity(DatetimeRange)`](crate::client::fluent_builders::CreateLicenseVersion::validity) / [`set_validity(Option<DatetimeRange>)`](crate::client::fluent_builders::CreateLicenseVersion::set_validity): <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    ///   - [`license_metadata(Vec<Metadata>)`](crate::client::fluent_builders::CreateLicenseVersion::license_metadata) / [`set_license_metadata(Option<Vec<Metadata>>)`](crate::client::fluent_builders::CreateLicenseVersion::set_license_metadata): <p>Information about the license.</p>
    ///   - [`entitlements(Vec<Entitlement>)`](crate::client::fluent_builders::CreateLicenseVersion::entitlements) / [`set_entitlements(Option<Vec<Entitlement>>)`](crate::client::fluent_builders::CreateLicenseVersion::set_entitlements): <p>License entitlements.</p>
    ///   - [`consumption_configuration(ConsumptionConfiguration)`](crate::client::fluent_builders::CreateLicenseVersion::consumption_configuration) / [`set_consumption_configuration(Option<ConsumptionConfiguration>)`](crate::client::fluent_builders::CreateLicenseVersion::set_consumption_configuration): <p>Configuration for consumption of the license. Choose a provisional configuration for workloads running with continuous connectivity. Choose a borrow configuration for workloads with offline usage.</p>
    ///   - [`status(LicenseStatus)`](crate::client::fluent_builders::CreateLicenseVersion::status) / [`set_status(Option<LicenseStatus>)`](crate::client::fluent_builders::CreateLicenseVersion::set_status): <p>License status.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    ///   - [`source_version(impl Into<String>)`](crate::client::fluent_builders::CreateLicenseVersion::source_version) / [`set_source_version(Option<String>)`](crate::client::fluent_builders::CreateLicenseVersion::set_source_version): <p>Current version of the license.</p>
                            /// - On success, responds with [`CreateLicenseVersionOutput`](crate::output::CreateLicenseVersionOutput) with field(s):
    ///   - [`license_arn(Option<String>)`](crate::output::CreateLicenseVersionOutput::license_arn): <p>License ARN.</p>
    ///   - [`version(Option<String>)`](crate::output::CreateLicenseVersionOutput::version): <p>New version of the license.</p>
    ///   - [`status(Option<LicenseStatus>)`](crate::output::CreateLicenseVersionOutput::status): <p>License status.</p>
                            /// - On failure, responds with [`SdkError<CreateLicenseVersionError>`](crate::error::CreateLicenseVersionError)
    pub fn create_license_version(&self) -> crate::client::fluent_builders::CreateLicenseVersion {
                                crate::client::fluent_builders::CreateLicenseVersion::new(self.handle.clone())
                            }
}

