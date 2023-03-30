// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDistributionsByResponseHeadersPolicyId`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::set_marker): <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::set_max_items): <p>The maximum number of distribution IDs that you want to get in the response.</p>
    ///   - [`response_headers_policy_id(impl Into<String>)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::response_headers_policy_id) / [`set_response_headers_policy_id(Option<String>)`](crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::set_response_headers_policy_id): <p>The ID of the response headers policy whose associated distribution IDs you want to list.</p>
                            /// - On success, responds with [`ListDistributionsByResponseHeadersPolicyIdOutput`](crate::output::ListDistributionsByResponseHeadersPolicyIdOutput) with field(s):
    ///   - [`distribution_id_list(Option<DistributionIdList>)`](crate::output::ListDistributionsByResponseHeadersPolicyIdOutput::distribution_id_list): <p>A list of distribution IDs.</p>
                            /// - On failure, responds with [`SdkError<ListDistributionsByResponseHeadersPolicyIdError>`](crate::error::ListDistributionsByResponseHeadersPolicyIdError)
    pub fn list_distributions_by_response_headers_policy_id(&self) -> crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId {
                                crate::client::fluent_builders::ListDistributionsByResponseHeadersPolicyId::new(self.handle.clone())
                            }
}

