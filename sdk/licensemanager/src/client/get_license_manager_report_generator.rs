// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLicenseManagerReportGenerator`](crate::client::fluent_builders::GetLicenseManagerReportGenerator) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`license_manager_report_generator_arn(impl Into<String>)`](crate::client::fluent_builders::GetLicenseManagerReportGenerator::license_manager_report_generator_arn) / [`set_license_manager_report_generator_arn(Option<String>)`](crate::client::fluent_builders::GetLicenseManagerReportGenerator::set_license_manager_report_generator_arn): <p>Amazon Resource Name (ARN) of the report generator.</p>
                            /// - On success, responds with [`GetLicenseManagerReportGeneratorOutput`](crate::output::GetLicenseManagerReportGeneratorOutput) with field(s):
    ///   - [`report_generator(Option<ReportGenerator>)`](crate::output::GetLicenseManagerReportGeneratorOutput::report_generator): <p>A report generator that creates periodic reports about your license configurations.</p>
                            /// - On failure, responds with [`SdkError<GetLicenseManagerReportGeneratorError>`](crate::error::GetLicenseManagerReportGeneratorError)
    pub fn get_license_manager_report_generator(&self) -> crate::client::fluent_builders::GetLicenseManagerReportGenerator {
                                crate::client::fluent_builders::GetLicenseManagerReportGenerator::new(self.handle.clone())
                            }
}

