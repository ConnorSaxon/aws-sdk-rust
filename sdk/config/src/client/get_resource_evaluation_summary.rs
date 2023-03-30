// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResourceEvaluationSummary`](crate::client::fluent_builders::GetResourceEvaluationSummary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_evaluation_id(impl Into<String>)`](crate::client::fluent_builders::GetResourceEvaluationSummary::resource_evaluation_id) / [`set_resource_evaluation_id(Option<String>)`](crate::client::fluent_builders::GetResourceEvaluationSummary::set_resource_evaluation_id): <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
                            /// - On success, responds with [`GetResourceEvaluationSummaryOutput`](crate::output::GetResourceEvaluationSummaryOutput) with field(s):
    ///   - [`resource_evaluation_id(Option<String>)`](crate::output::GetResourceEvaluationSummaryOutput::resource_evaluation_id): <p>The unique <code>ResourceEvaluationId</code> of Amazon Web Services resource execution for which you want to retrieve the evaluation summary.</p>
    ///   - [`evaluation_mode(Option<EvaluationMode>)`](crate::output::GetResourceEvaluationSummaryOutput::evaluation_mode): <p>Lists results of the mode that you requested to retrieve the resource evaluation summary. The valid values are Detective or Proactive.</p>
    ///   - [`evaluation_status(Option<EvaluationStatus>)`](crate::output::GetResourceEvaluationSummaryOutput::evaluation_status): <p>Returns an <code>EvaluationStatus</code> object.</p>
    ///   - [`evaluation_start_timestamp(Option<DateTime>)`](crate::output::GetResourceEvaluationSummaryOutput::evaluation_start_timestamp): <p>The start timestamp when Config rule starts evaluating compliance for the provided resource details.</p>
    ///   - [`compliance(Option<ComplianceType>)`](crate::output::GetResourceEvaluationSummaryOutput::compliance): <p>The compliance status of the resource evaluation summary.</p>
    ///   - [`evaluation_context(Option<EvaluationContext>)`](crate::output::GetResourceEvaluationSummaryOutput::evaluation_context): <p>Returns an <code>EvaluationContext</code> object.</p>
    ///   - [`resource_details(Option<ResourceDetails>)`](crate::output::GetResourceEvaluationSummaryOutput::resource_details): <p>Returns a <code>ResourceDetails</code> object.</p>
                            /// - On failure, responds with [`SdkError<GetResourceEvaluationSummaryError>`](crate::error::GetResourceEvaluationSummaryError)
    pub fn get_resource_evaluation_summary(&self) -> crate::client::fluent_builders::GetResourceEvaluationSummary {
                                crate::client::fluent_builders::GetResourceEvaluationSummary::new(self.handle.clone())
                            }
}

