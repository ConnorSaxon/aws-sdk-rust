// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetConformancePackComplianceDetails`](crate::client::fluent_builders::GetConformancePackComplianceDetails) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetConformancePackComplianceDetails::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`conformance_pack_name(impl Into<String>)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::conformance_pack_name) / [`set_conformance_pack_name(Option<String>)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::set_conformance_pack_name): <p>Name of the conformance pack.</p>
    ///   - [`filters(ConformancePackEvaluationFilters)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::filters) / [`set_filters(Option<ConformancePackEvaluationFilters>)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::set_filters): <p>A <code>ConformancePackEvaluationFilters</code> object.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::limit) / [`set_limit(i32)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::set_limit): <p>The maximum number of evaluation results returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetConformancePackComplianceDetails::set_next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
                            /// - On success, responds with [`GetConformancePackComplianceDetailsOutput`](crate::output::GetConformancePackComplianceDetailsOutput) with field(s):
    ///   - [`conformance_pack_name(Option<String>)`](crate::output::GetConformancePackComplianceDetailsOutput::conformance_pack_name): <p>Name of the conformance pack.</p>
    ///   - [`conformance_pack_rule_evaluation_results(Option<Vec<ConformancePackEvaluationResult>>)`](crate::output::GetConformancePackComplianceDetailsOutput::conformance_pack_rule_evaluation_results): <p>Returns a list of <code>ConformancePackEvaluationResult</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetConformancePackComplianceDetailsOutput::next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
                            /// - On failure, responds with [`SdkError<GetConformancePackComplianceDetailsError>`](crate::error::GetConformancePackComplianceDetailsError)
    pub fn get_conformance_pack_compliance_details(&self) -> crate::client::fluent_builders::GetConformancePackComplianceDetails {
                                crate::client::fluent_builders::GetConformancePackComplianceDetails::new(self.handle.clone())
                            }
}

