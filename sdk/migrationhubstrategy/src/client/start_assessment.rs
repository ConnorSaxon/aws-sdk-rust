// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartAssessment`](crate::client::fluent_builders::StartAssessment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`s3bucket_for_analysis_data(impl Into<String>)`](crate::client::fluent_builders::StartAssessment::s3bucket_for_analysis_data) / [`set_s3bucket_for_analysis_data(Option<String>)`](crate::client::fluent_builders::StartAssessment::set_s3bucket_for_analysis_data): <p> The S3 bucket used by the collectors to send analysis data to the service. The bucket name must begin with <code>migrationhub-strategy-</code>. </p>
    ///   - [`s3bucket_for_report_data(impl Into<String>)`](crate::client::fluent_builders::StartAssessment::s3bucket_for_report_data) / [`set_s3bucket_for_report_data(Option<String>)`](crate::client::fluent_builders::StartAssessment::set_s3bucket_for_report_data): <p> The S3 bucket where all the reports generated by the service are stored. The bucket name must begin with <code>migrationhub-strategy-</code>. </p>
    ///   - [`assessment_targets(Vec<AssessmentTarget>)`](crate::client::fluent_builders::StartAssessment::assessment_targets) / [`set_assessment_targets(Option<Vec<AssessmentTarget>>)`](crate::client::fluent_builders::StartAssessment::set_assessment_targets): <p>List of criteria for assessment.</p>
                            /// - On success, responds with [`StartAssessmentOutput`](crate::output::StartAssessmentOutput) with field(s):
    ///   - [`assessment_id(Option<String>)`](crate::output::StartAssessmentOutput::assessment_id): <p> The ID of the assessment. </p>
                            /// - On failure, responds with [`SdkError<StartAssessmentError>`](crate::error::StartAssessmentError)
    pub fn start_assessment(&self) -> crate::client::fluent_builders::StartAssessment {
                                crate::client::fluent_builders::StartAssessment::new(self.handle.clone())
                            }
}

