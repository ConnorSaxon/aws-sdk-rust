// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateBusinessReportSchedule`](crate::client::fluent_builders::UpdateBusinessReportSchedule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schedule_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::schedule_arn) / [`set_schedule_arn(Option<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_schedule_arn): <p>The ARN of the business report schedule.</p>
    ///   - [`s3_bucket_name(impl Into<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::s3_bucket_name) / [`set_s3_bucket_name(Option<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_s3_bucket_name): <p>The S3 location of the output reports.</p>
    ///   - [`s3_key_prefix(impl Into<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::s3_key_prefix) / [`set_s3_key_prefix(Option<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_s3_key_prefix): <p>The S3 key where the report is delivered.</p>
    ///   - [`format(BusinessReportFormat)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::format) / [`set_format(Option<BusinessReportFormat>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_format): <p>The format of the generated report (individual CSV files or zipped files of individual files).</p>
    ///   - [`schedule_name(impl Into<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::schedule_name) / [`set_schedule_name(Option<String>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_schedule_name): <p>The name identifier of the schedule.</p>
    ///   - [`recurrence(BusinessReportRecurrence)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::recurrence) / [`set_recurrence(Option<BusinessReportRecurrence>)`](crate::client::fluent_builders::UpdateBusinessReportSchedule::set_recurrence): <p>The recurrence of the reports.</p>
                            /// - On success, responds with [`UpdateBusinessReportScheduleOutput`](crate::output::UpdateBusinessReportScheduleOutput)
                            /// - On failure, responds with [`SdkError<UpdateBusinessReportScheduleError>`](crate::error::UpdateBusinessReportScheduleError)
    pub fn update_business_report_schedule(&self) -> crate::client::fluent_builders::UpdateBusinessReportSchedule {
                                crate::client::fluent_builders::UpdateBusinessReportSchedule::new(self.handle.clone())
                            }
}

