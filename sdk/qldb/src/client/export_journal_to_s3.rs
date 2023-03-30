// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ExportJournalToS3`](crate::client::fluent_builders::ExportJournalToS3) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::ExportJournalToS3::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::ExportJournalToS3::set_name): <p>The name of the ledger.</p>
    ///   - [`inclusive_start_time(DateTime)`](crate::client::fluent_builders::ExportJournalToS3::inclusive_start_time) / [`set_inclusive_start_time(Option<DateTime>)`](crate::client::fluent_builders::ExportJournalToS3::set_inclusive_start_time): <p>The inclusive start date and time for the range of journal contents to export.</p>  <p>The <code>InclusiveStartTime</code> must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code>.</p>  <p>The <code>InclusiveStartTime</code> must be before <code>ExclusiveEndTime</code>.</p>  <p>If you provide an <code>InclusiveStartTime</code> that is before the ledger's <code>CreationDateTime</code>, Amazon QLDB defaults it to the ledger's <code>CreationDateTime</code>.</p>
    ///   - [`exclusive_end_time(DateTime)`](crate::client::fluent_builders::ExportJournalToS3::exclusive_end_time) / [`set_exclusive_end_time(Option<DateTime>)`](crate::client::fluent_builders::ExportJournalToS3::set_exclusive_end_time): <p>The exclusive end date and time for the range of journal contents to export.</p>  <p>The <code>ExclusiveEndTime</code> must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code>.</p>  <p>The <code>ExclusiveEndTime</code> must be less than or equal to the current UTC date and time.</p>
    ///   - [`s3_export_configuration(S3ExportConfiguration)`](crate::client::fluent_builders::ExportJournalToS3::s3_export_configuration) / [`set_s3_export_configuration(Option<S3ExportConfiguration>)`](crate::client::fluent_builders::ExportJournalToS3::set_s3_export_configuration): <p>The configuration settings of the Amazon S3 bucket destination for your export request.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::ExportJournalToS3::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::ExportJournalToS3::set_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal export job to do the following:</p>  <ul>   <li> <p>Write objects into your Amazon Simple Storage Service (Amazon S3) bucket.</p> </li>   <li> <p>(Optional) Use your customer managed key in Key Management Service (KMS) for server-side encryption of your exported data.</p> </li>  </ul>  <p>To pass a role to QLDB when requesting a journal export, you must have permissions to perform the <code>iam:PassRole</code> action on the IAM role resource. This is required for all journal export requests.</p>
    ///   - [`output_format(OutputFormat)`](crate::client::fluent_builders::ExportJournalToS3::output_format) / [`set_output_format(Option<OutputFormat>)`](crate::client::fluent_builders::ExportJournalToS3::set_output_format): <p>The output format of your exported journal data. If this parameter is not specified, the exported data defaults to <code>ION_TEXT</code> format.</p>
                            /// - On success, responds with [`ExportJournalToS3Output`](crate::output::ExportJournalToS3Output) with field(s):
    ///   - [`export_id(Option<String>)`](crate::output::ExportJournalToS3Output::export_id): <p>The UUID (represented in Base62-encoded text) that QLDB assigns to each journal export job.</p>  <p>To describe your export request and check the status of the job, you can use <code>ExportId</code> to call <code>DescribeJournalS3Export</code>.</p>
                            /// - On failure, responds with [`SdkError<ExportJournalToS3Error>`](crate::error::ExportJournalToS3Error)
    pub fn export_journal_to_s3(&self) -> crate::client::fluent_builders::ExportJournalToS3 {
                                crate::client::fluent_builders::ExportJournalToS3::new(self.handle.clone())
                            }
}

