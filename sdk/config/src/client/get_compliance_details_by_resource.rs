// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetComplianceDetailsByResource`](crate::client::fluent_builders::GetComplianceDetailsByResource) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetComplianceDetailsByResource::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_type(impl Into<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::resource_type) / [`set_resource_type(Option<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::set_resource_type): <p>The type of the Amazon Web Services resource for which you want compliance information.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::set_resource_id): <p>The ID of the Amazon Web Services resource for which you want compliance information.</p>
    ///   - [`compliance_types(Vec<ComplianceType>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::compliance_types) / [`set_compliance_types(Option<Vec<ComplianceType>>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::set_compliance_types): <p>Filters the results by compliance.</p>  <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    ///   - [`resource_evaluation_id(impl Into<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::resource_evaluation_id) / [`set_resource_evaluation_id(Option<String>)`](crate::client::fluent_builders::GetComplianceDetailsByResource::set_resource_evaluation_id): <p>The unique ID of Amazon Web Services resource execution for which you want to retrieve evaluation results. </p> <note>   <p>You need to only provide either a <code>ResourceEvaluationID</code> or a <code>ResourceID </code>and <code>ResourceType</code>.</p>  </note>
                            /// - On success, responds with [`GetComplianceDetailsByResourceOutput`](crate::output::GetComplianceDetailsByResourceOutput) with field(s):
    ///   - [`evaluation_results(Option<Vec<EvaluationResult>>)`](crate::output::GetComplianceDetailsByResourceOutput::evaluation_results): <p>Indicates whether the specified Amazon Web Services resource complies each Config rule.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetComplianceDetailsByResourceOutput::next_token): <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
                            /// - On failure, responds with [`SdkError<GetComplianceDetailsByResourceError>`](crate::error::GetComplianceDetailsByResourceError)
    pub fn get_compliance_details_by_resource(&self) -> crate::client::fluent_builders::GetComplianceDetailsByResource {
                                crate::client::fluent_builders::GetComplianceDetailsByResource::new(self.handle.clone())
                            }
}

