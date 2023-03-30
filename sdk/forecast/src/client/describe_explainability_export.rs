// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeExplainabilityExport`](crate::client::fluent_builders::DescribeExplainabilityExport) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`explainability_export_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeExplainabilityExport::explainability_export_arn) / [`set_explainability_export_arn(Option<String>)`](crate::client::fluent_builders::DescribeExplainabilityExport::set_explainability_export_arn): <p>The Amazon Resource Name (ARN) of the Explainability export.</p>
                            /// - On success, responds with [`DescribeExplainabilityExportOutput`](crate::output::DescribeExplainabilityExportOutput) with field(s):
    ///   - [`explainability_export_arn(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::explainability_export_arn): <p>The Amazon Resource Name (ARN) of the Explainability export.</p>
    ///   - [`explainability_export_name(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::explainability_export_name): <p>The name of the Explainability export.</p>
    ///   - [`explainability_arn(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::explainability_arn): <p>The Amazon Resource Name (ARN) of the Explainability export.</p>
    ///   - [`destination(Option<DataDestination>)`](crate::output::DescribeExplainabilityExportOutput::destination): <p>The destination for an export job. Provide an S3 path, an AWS Identity and Access Management (IAM) role that allows Amazon Forecast to access the location, and an AWS Key Management Service (KMS) key (optional). </p>
    ///   - [`message(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::message): <p>Information about any errors that occurred during the export.</p>
    ///   - [`status(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::status): <p>The status of the Explainability export. States include: </p>  <ul>   <li> <p> <code>ACTIVE</code> </p> </li>   <li> <p> <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>, <code>CREATE_FAILED</code> </p> </li>   <li> <p> <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code> </p> </li>   <li> <p> <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>, <code>DELETE_FAILED</code> </p> </li>  </ul>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeExplainabilityExportOutput::creation_time): <p>When the Explainability export was created.</p>
    ///   - [`last_modification_time(Option<DateTime>)`](crate::output::DescribeExplainabilityExportOutput::last_modification_time): <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>  <ul>   <li> <p> <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p> </li>   <li> <p> <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p> </li>   <li> <p> <code>CREATE_STOPPING</code> - The current timestamp.</p> </li>   <li> <p> <code>CREATE_STOPPED</code> - When the job stopped.</p> </li>   <li> <p> <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or failed.</p> </li>  </ul>
    ///   - [`format(Option<String>)`](crate::output::DescribeExplainabilityExportOutput::format): <p>The format of the exported data, CSV or PARQUET.</p>
                            /// - On failure, responds with [`SdkError<DescribeExplainabilityExportError>`](crate::error::DescribeExplainabilityExportError)
    pub fn describe_explainability_export(&self) -> crate::client::fluent_builders::DescribeExplainabilityExport {
                                crate::client::fluent_builders::DescribeExplainabilityExport::new(self.handle.clone())
                            }
}

