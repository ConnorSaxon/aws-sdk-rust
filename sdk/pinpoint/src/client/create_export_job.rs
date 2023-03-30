// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateExportJob`](crate::client::fluent_builders::CreateExportJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::CreateExportJob::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::CreateExportJob::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`export_job_request(ExportJobRequest)`](crate::client::fluent_builders::CreateExportJob::export_job_request) / [`set_export_job_request(Option<ExportJobRequest>)`](crate::client::fluent_builders::CreateExportJob::set_export_job_request): <p>Specifies the settings for a job that exports endpoint definitions to an Amazon Simple Storage Service (Amazon S3) bucket.</p>
                            /// - On success, responds with [`CreateExportJobOutput`](crate::output::CreateExportJobOutput) with field(s):
    ///   - [`export_job_response(Option<ExportJobResponse>)`](crate::output::CreateExportJobOutput::export_job_response): <p>Provides information about the status and settings of a job that exports endpoint definitions to a file. The file can be added directly to an Amazon Simple Storage Service (Amazon S3) bucket by using the Amazon Pinpoint API or downloaded directly to a computer by using the Amazon Pinpoint console.</p>
                            /// - On failure, responds with [`SdkError<CreateExportJobError>`](crate::error::CreateExportJobError)
    pub fn create_export_job(&self) -> crate::client::fluent_builders::CreateExportJob {
                                crate::client::fluent_builders::CreateExportJob::new(self.handle.clone())
                            }
}

