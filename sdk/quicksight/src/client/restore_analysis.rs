// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RestoreAnalysis`](crate::client::fluent_builders::RestoreAnalysis) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::RestoreAnalysis::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::RestoreAnalysis::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the analysis.</p>
    ///   - [`analysis_id(impl Into<String>)`](crate::client::fluent_builders::RestoreAnalysis::analysis_id) / [`set_analysis_id(Option<String>)`](crate::client::fluent_builders::RestoreAnalysis::set_analysis_id): <p>The ID of the analysis that you're restoring.</p>
                            /// - On success, responds with [`RestoreAnalysisOutput`](crate::output::RestoreAnalysisOutput) with field(s):
    ///   - [`status(i32)`](crate::output::RestoreAnalysisOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`arn(Option<String>)`](crate::output::RestoreAnalysisOutput::arn): <p>The Amazon Resource Name (ARN) of the analysis that you're restoring.</p>
    ///   - [`analysis_id(Option<String>)`](crate::output::RestoreAnalysisOutput::analysis_id): <p>The ID of the analysis that you're restoring. </p>
    ///   - [`request_id(Option<String>)`](crate::output::RestoreAnalysisOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<RestoreAnalysisError>`](crate::error::RestoreAnalysisError)
    pub fn restore_analysis(&self) -> crate::client::fluent_builders::RestoreAnalysis {
                                crate::client::fluent_builders::RestoreAnalysis::new(self.handle.clone())
                            }
}

