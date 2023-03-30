// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLendingAnalysisSummary`](crate::client::fluent_builders::GetLendingAnalysisSummary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::GetLendingAnalysisSummary::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::GetLendingAnalysisSummary::set_job_id): <p> A unique identifier for the lending or text-detection job. The <code>JobId</code> is returned from StartLendingAnalysis. A <code>JobId</code> value is only valid for 7 days.</p>
                            /// - On success, responds with [`GetLendingAnalysisSummaryOutput`](crate::output::GetLendingAnalysisSummaryOutput) with field(s):
    ///   - [`document_metadata(Option<DocumentMetadata>)`](crate::output::GetLendingAnalysisSummaryOutput::document_metadata): <p>Information about the input document.</p>
    ///   - [`job_status(Option<JobStatus>)`](crate::output::GetLendingAnalysisSummaryOutput::job_status): <p> The current status of the lending analysis job. </p>
    ///   - [`summary(Option<LendingSummary>)`](crate::output::GetLendingAnalysisSummaryOutput::summary): <p> Contains summary information for documents grouped by type.</p>
    ///   - [`warnings(Option<Vec<Warning>>)`](crate::output::GetLendingAnalysisSummaryOutput::warnings): <p>A list of warnings that occurred during the lending analysis operation.</p>
    ///   - [`status_message(Option<String>)`](crate::output::GetLendingAnalysisSummaryOutput::status_message): <p>Returns if the lending analysis could not be completed. Contains explanation for what error occurred.</p>
    ///   - [`analyze_lending_model_version(Option<String>)`](crate::output::GetLendingAnalysisSummaryOutput::analyze_lending_model_version): <p>The current model version of the Analyze Lending API.</p>
                            /// - On failure, responds with [`SdkError<GetLendingAnalysisSummaryError>`](crate::error::GetLendingAnalysisSummaryError)
    pub fn get_lending_analysis_summary(&self) -> crate::client::fluent_builders::GetLendingAnalysisSummary {
                                crate::client::fluent_builders::GetLendingAnalysisSummary::new(self.handle.clone())
                            }
}

