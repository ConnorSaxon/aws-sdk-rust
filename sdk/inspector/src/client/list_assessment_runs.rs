// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAssessmentRuns`](crate::client::fluent_builders::ListAssessmentRuns) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAssessmentRuns::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`assessment_template_arns(Vec<String>)`](crate::client::fluent_builders::ListAssessmentRuns::assessment_template_arns) / [`set_assessment_template_arns(Option<Vec<String>>)`](crate::client::fluent_builders::ListAssessmentRuns::set_assessment_template_arns): <p>The ARNs that specify the assessment templates whose assessment runs you want to list.</p>
    ///   - [`filter(AssessmentRunFilter)`](crate::client::fluent_builders::ListAssessmentRuns::filter) / [`set_filter(Option<AssessmentRunFilter>)`](crate::client::fluent_builders::ListAssessmentRuns::set_filter): <p>You can use this parameter to specify a subset of data to be included in the action's response.</p>  <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAssessmentRuns::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAssessmentRuns::set_next_token): <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentRuns</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAssessmentRuns::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAssessmentRuns::set_max_results): <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 10. The maximum value is 500.</p>
                            /// - On success, responds with [`ListAssessmentRunsOutput`](crate::output::ListAssessmentRunsOutput) with field(s):
    ///   - [`assessment_run_arns(Option<Vec<String>>)`](crate::output::ListAssessmentRunsOutput::assessment_run_arns): <p>A list of ARNs that specifies the assessment runs that are returned by the action.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAssessmentRunsOutput::next_token): <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
                            /// - On failure, responds with [`SdkError<ListAssessmentRunsError>`](crate::error::ListAssessmentRunsError)
    pub fn list_assessment_runs(&self) -> crate::client::fluent_builders::ListAssessmentRuns {
                                crate::client::fluent_builders::ListAssessmentRuns::new(self.handle.clone())
                            }
}

