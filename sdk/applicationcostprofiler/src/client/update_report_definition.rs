// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateReportDefinition`](crate::client::fluent_builders::UpdateReportDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`report_id(impl Into<String>)`](crate::client::fluent_builders::UpdateReportDefinition::report_id) / [`set_report_id(Option<String>)`](crate::client::fluent_builders::UpdateReportDefinition::set_report_id): <p>Required. ID of the report to update.</p>
    ///   - [`report_description(impl Into<String>)`](crate::client::fluent_builders::UpdateReportDefinition::report_description) / [`set_report_description(Option<String>)`](crate::client::fluent_builders::UpdateReportDefinition::set_report_description): <p>Required. Description of the report.</p>
    ///   - [`report_frequency(ReportFrequency)`](crate::client::fluent_builders::UpdateReportDefinition::report_frequency) / [`set_report_frequency(Option<ReportFrequency>)`](crate::client::fluent_builders::UpdateReportDefinition::set_report_frequency): <p>Required. The cadence to generate the report.</p>
    ///   - [`format(Format)`](crate::client::fluent_builders::UpdateReportDefinition::format) / [`set_format(Option<Format>)`](crate::client::fluent_builders::UpdateReportDefinition::set_format): <p>Required. The format to use for the generated report.</p>
    ///   - [`destination_s3_location(S3Location)`](crate::client::fluent_builders::UpdateReportDefinition::destination_s3_location) / [`set_destination_s3_location(Option<S3Location>)`](crate::client::fluent_builders::UpdateReportDefinition::set_destination_s3_location): <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the report.</p>
                            /// - On success, responds with [`UpdateReportDefinitionOutput`](crate::output::UpdateReportDefinitionOutput) with field(s):
    ///   - [`report_id(Option<String>)`](crate::output::UpdateReportDefinitionOutput::report_id): <p>ID of the report.</p>
                            /// - On failure, responds with [`SdkError<UpdateReportDefinitionError>`](crate::error::UpdateReportDefinitionError)
    pub fn update_report_definition(&self) -> crate::client::fluent_builders::UpdateReportDefinition {
                                crate::client::fluent_builders::UpdateReportDefinition::new(self.handle.clone())
                            }
}

