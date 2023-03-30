// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetFindingsReportStatus`](crate::client::fluent_builders::GetFindingsReportStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`report_id(impl Into<String>)`](crate::client::fluent_builders::GetFindingsReportStatus::report_id) / [`set_report_id(Option<String>)`](crate::client::fluent_builders::GetFindingsReportStatus::set_report_id): <p>The ID of the report to retrieve the status of.</p>
                            /// - On success, responds with [`GetFindingsReportStatusOutput`](crate::output::GetFindingsReportStatusOutput) with field(s):
    ///   - [`report_id(Option<String>)`](crate::output::GetFindingsReportStatusOutput::report_id): <p>The ID of the report.</p>
    ///   - [`status(Option<ExternalReportStatus>)`](crate::output::GetFindingsReportStatusOutput::status): <p>The status of the report.</p>
    ///   - [`error_code(Option<ReportingErrorCode>)`](crate::output::GetFindingsReportStatusOutput::error_code): <p>The error code of the report.</p>
    ///   - [`error_message(Option<String>)`](crate::output::GetFindingsReportStatusOutput::error_message): <p>The error message of the report.</p>
    ///   - [`destination(Option<Destination>)`](crate::output::GetFindingsReportStatusOutput::destination): <p>The destination of the report.</p>
    ///   - [`filter_criteria(Option<FilterCriteria>)`](crate::output::GetFindingsReportStatusOutput::filter_criteria): <p>The filter criteria associated with the report.</p>
                            /// - On failure, responds with [`SdkError<GetFindingsReportStatusError>`](crate::error::GetFindingsReportStatusError)
    pub fn get_findings_report_status(&self) -> crate::client::fluent_builders::GetFindingsReportStatus {
                                crate::client::fluent_builders::GetFindingsReportStatus::new(self.handle.clone())
                            }
}

