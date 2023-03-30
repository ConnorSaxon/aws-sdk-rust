// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCostAllocationTags`](crate::client::fluent_builders::ListCostAllocationTags) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCostAllocationTags::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`status(CostAllocationTagStatus)`](crate::client::fluent_builders::ListCostAllocationTags::status) / [`set_status(Option<CostAllocationTagStatus>)`](crate::client::fluent_builders::ListCostAllocationTags::set_status): <p>The status of cost allocation tag keys that are returned for this request. </p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::ListCostAllocationTags::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::ListCostAllocationTags::set_tag_keys): <p>The list of cost allocation tag keys that are returned for this request. </p>
    ///   - [`r#type(CostAllocationTagType)`](crate::client::fluent_builders::ListCostAllocationTags::type) / [`set_type(Option<CostAllocationTagType>)`](crate::client::fluent_builders::ListCostAllocationTags::set_type): <p>The type of <code>CostAllocationTag</code> object that are returned for this request. The <code>AWSGenerated</code> type tags are tags that Amazon Web Services defines and applies to support Amazon Web Services resources for cost allocation purposes. The <code>UserDefined</code> type tags are tags that you define, create, and apply to resources. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCostAllocationTags::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCostAllocationTags::set_next_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCostAllocationTags::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCostAllocationTags::set_max_results): <p>The maximum number of objects that are returned for this request. By default, the request returns 100 results. </p>
                            /// - On success, responds with [`ListCostAllocationTagsOutput`](crate::output::ListCostAllocationTagsOutput) with field(s):
    ///   - [`cost_allocation_tags(Option<Vec<CostAllocationTag>>)`](crate::output::ListCostAllocationTagsOutput::cost_allocation_tags): <p>A list of cost allocation tags that includes the detailed metadata for each one. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListCostAllocationTagsOutput::next_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
                            /// - On failure, responds with [`SdkError<ListCostAllocationTagsError>`](crate::error::ListCostAllocationTagsError)
    pub fn list_cost_allocation_tags(&self) -> crate::client::fluent_builders::ListCostAllocationTags {
                                crate::client::fluent_builders::ListCostAllocationTags::new(self.handle.clone())
                            }
}

