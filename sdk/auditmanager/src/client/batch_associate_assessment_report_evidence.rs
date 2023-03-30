// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchAssociateAssessmentReportEvidence`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`assessment_id(impl Into<String>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::assessment_id) / [`set_assessment_id(Option<String>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::set_assessment_id): <p> The identifier for the assessment. </p>
    ///   - [`evidence_folder_id(impl Into<String>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::evidence_folder_id) / [`set_evidence_folder_id(Option<String>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::set_evidence_folder_id): <p> The identifier for the folder that the evidence is stored in. </p>
    ///   - [`evidence_ids(Vec<String>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::evidence_ids) / [`set_evidence_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::set_evidence_ids): <p> The list of evidence identifiers. </p>
                            /// - On success, responds with [`BatchAssociateAssessmentReportEvidenceOutput`](crate::output::BatchAssociateAssessmentReportEvidenceOutput) with field(s):
    ///   - [`evidence_ids(Option<Vec<String>>)`](crate::output::BatchAssociateAssessmentReportEvidenceOutput::evidence_ids): <p> The list of evidence identifiers. </p>
    ///   - [`errors(Option<Vec<AssessmentReportEvidenceError>>)`](crate::output::BatchAssociateAssessmentReportEvidenceOutput::errors): <p> A list of errors that the <code>BatchAssociateAssessmentReportEvidence</code> API returned. </p>
                            /// - On failure, responds with [`SdkError<BatchAssociateAssessmentReportEvidenceError>`](crate::error::BatchAssociateAssessmentReportEvidenceError)
    pub fn batch_associate_assessment_report_evidence(&self) -> crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence {
                                crate::client::fluent_builders::BatchAssociateAssessmentReportEvidence::new(self.handle.clone())
                            }
}

