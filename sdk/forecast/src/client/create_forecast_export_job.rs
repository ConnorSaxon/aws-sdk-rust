// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateForecastExportJob`](crate::client::fluent_builders::CreateForecastExportJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`forecast_export_job_name(impl Into<String>)`](crate::client::fluent_builders::CreateForecastExportJob::forecast_export_job_name) / [`set_forecast_export_job_name(Option<String>)`](crate::client::fluent_builders::CreateForecastExportJob::set_forecast_export_job_name): <p>The name for the forecast export job.</p>
    ///   - [`forecast_arn(impl Into<String>)`](crate::client::fluent_builders::CreateForecastExportJob::forecast_arn) / [`set_forecast_arn(Option<String>)`](crate::client::fluent_builders::CreateForecastExportJob::set_forecast_arn): <p>The Amazon Resource Name (ARN) of the forecast that you want to export.</p>
    ///   - [`destination(DataDestination)`](crate::client::fluent_builders::CreateForecastExportJob::destination) / [`set_destination(Option<DataDestination>)`](crate::client::fluent_builders::CreateForecastExportJob::set_destination): <p>The location where you want to save the forecast and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>  <p>If encryption is used, <code>Destination</code> must include an AWS Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateForecastExportJob::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateForecastExportJob::set_tags): <p>The optional metadata that you apply to the forecast export job to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>  <p>The following basic restrictions apply to tags:</p>  <ul>   <li> <p>Maximum number of tags per resource - 50.</p> </li>   <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>   <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li>   <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li>   <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>   <li> <p>Tag keys and values are case sensitive.</p> </li>   <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li>  </ul>
    ///   - [`format(impl Into<String>)`](crate::client::fluent_builders::CreateForecastExportJob::format) / [`set_format(Option<String>)`](crate::client::fluent_builders::CreateForecastExportJob::set_format): <p>The format of the exported data, CSV or PARQUET. The default value is CSV.</p>
                            /// - On success, responds with [`CreateForecastExportJobOutput`](crate::output::CreateForecastExportJobOutput) with field(s):
    ///   - [`forecast_export_job_arn(Option<String>)`](crate::output::CreateForecastExportJobOutput::forecast_export_job_arn): <p>The Amazon Resource Name (ARN) of the export job.</p>
                            /// - On failure, responds with [`SdkError<CreateForecastExportJobError>`](crate::error::CreateForecastExportJobError)
    pub fn create_forecast_export_job(&self) -> crate::client::fluent_builders::CreateForecastExportJob {
                                crate::client::fluent_builders::CreateForecastExportJob::new(self.handle.clone())
                            }
}

