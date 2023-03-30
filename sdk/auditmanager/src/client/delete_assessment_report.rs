// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAssessmentReport`](crate::client::fluent_builders::DeleteAssessmentReport) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`assessment_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAssessmentReport::assessment_id) / [`set_assessment_id(Option<String>)`](crate::client::fluent_builders::DeleteAssessmentReport::set_assessment_id): <p> The unique identifier for the assessment. </p>
    ///   - [`assessment_report_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAssessmentReport::assessment_report_id) / [`set_assessment_report_id(Option<String>)`](crate::client::fluent_builders::DeleteAssessmentReport::set_assessment_report_id): <p> The unique identifier for the assessment report. </p>
                            /// - On success, responds with [`DeleteAssessmentReportOutput`](crate::output::DeleteAssessmentReportOutput)
                            /// - On failure, responds with [`SdkError<DeleteAssessmentReportError>`](crate::error::DeleteAssessmentReportError)
    pub fn delete_assessment_report(&self) -> crate::client::fluent_builders::DeleteAssessmentReport {
                                crate::client::fluent_builders::DeleteAssessmentReport::new(self.handle.clone())
                            }
}

