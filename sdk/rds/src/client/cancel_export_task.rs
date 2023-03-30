// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelExportTask`](crate::client::fluent_builders::CancelExportTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`export_task_identifier(impl Into<String>)`](crate::client::fluent_builders::CancelExportTask::export_task_identifier) / [`set_export_task_identifier(Option<String>)`](crate::client::fluent_builders::CancelExportTask::set_export_task_identifier): <p>The identifier of the snapshot export task to cancel.</p>
                            /// - On success, responds with [`CancelExportTaskOutput`](crate::output::CancelExportTaskOutput) with field(s):
    ///   - [`export_task_identifier(Option<String>)`](crate::output::CancelExportTaskOutput::export_task_identifier): <p>A unique identifier for the snapshot export task. This ID isn't an identifier for the Amazon S3 bucket where the snapshot is exported to.</p>
    ///   - [`source_arn(Option<String>)`](crate::output::CancelExportTaskOutput::source_arn): <p>The Amazon Resource Name (ARN) of the snapshot exported to Amazon S3.</p>
    ///   - [`export_only(Option<Vec<String>>)`](crate::output::CancelExportTaskOutput::export_only): <p>The data exported from the snapshot. Valid values are the following:</p>  <ul>   <li> <p> <code>database</code> - Export all the data from a specified database.</p> </li>   <li> <p> <code>database.table</code> <i>table-name</i> - Export a table of the snapshot. This format is valid only for RDS for MySQL, RDS for MariaDB, and Aurora MySQL.</p> </li>   <li> <p> <code>database.schema</code> <i>schema-name</i> - Export a database schema of the snapshot. This format is valid only for RDS for PostgreSQL and Aurora PostgreSQL.</p> </li>   <li> <p> <code>database.schema.table</code> <i>table-name</i> - Export a table of the database schema. This format is valid only for RDS for PostgreSQL and Aurora PostgreSQL.</p> </li>  </ul>
    ///   - [`snapshot_time(Option<DateTime>)`](crate::output::CancelExportTaskOutput::snapshot_time): <p>The time that the snapshot was created.</p>
    ///   - [`task_start_time(Option<DateTime>)`](crate::output::CancelExportTaskOutput::task_start_time): <p>The time that the snapshot export task started.</p>
    ///   - [`task_end_time(Option<DateTime>)`](crate::output::CancelExportTaskOutput::task_end_time): <p>The time that the snapshot export task completed.</p>
    ///   - [`s3_bucket(Option<String>)`](crate::output::CancelExportTaskOutput::s3_bucket): <p>The Amazon S3 bucket that the snapshot is exported to.</p>
    ///   - [`s3_prefix(Option<String>)`](crate::output::CancelExportTaskOutput::s3_prefix): <p>The Amazon S3 bucket prefix that is the file name and path of the exported snapshot.</p>
    ///   - [`iam_role_arn(Option<String>)`](crate::output::CancelExportTaskOutput::iam_role_arn): <p>The name of the IAM role that is used to write to Amazon S3 when exporting a snapshot.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::output::CancelExportTaskOutput::kms_key_id): <p>The key identifier of the Amazon Web Services KMS key that is used to encrypt the snapshot when it's exported to Amazon S3. The KMS key identifier is its key ARN, key ID, alias ARN, or alias name. The IAM role used for the snapshot export must have encryption and decryption permissions to use this KMS key.</p>
    ///   - [`status(Option<String>)`](crate::output::CancelExportTaskOutput::status): <p>The progress status of the export task.</p>
    ///   - [`percent_progress(i32)`](crate::output::CancelExportTaskOutput::percent_progress): <p>The progress of the snapshot export task as a percentage.</p>
    ///   - [`total_extracted_data_in_gb(i32)`](crate::output::CancelExportTaskOutput::total_extracted_data_in_gb): <p>The total amount of data exported, in gigabytes.</p>
    ///   - [`failure_cause(Option<String>)`](crate::output::CancelExportTaskOutput::failure_cause): <p>The reason the export failed, if it failed.</p>
    ///   - [`warning_message(Option<String>)`](crate::output::CancelExportTaskOutput::warning_message): <p>A warning about the snapshot export task.</p>
    ///   - [`source_type(Option<ExportSourceType>)`](crate::output::CancelExportTaskOutput::source_type): <p>The type of source for the export.</p>
                            /// - On failure, responds with [`SdkError<CancelExportTaskError>`](crate::error::CancelExportTaskError)
    pub fn cancel_export_task(&self) -> crate::client::fluent_builders::CancelExportTask {
                                crate::client::fluent_builders::CancelExportTask::new(self.handle.clone())
                            }
}

