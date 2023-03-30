// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetComplianceSummary`](crate::client::fluent_builders::GetComplianceSummary) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetComplianceSummary::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`target_id_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::target_id_filters) / [`set_target_id_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_target_id_filters): <p>Specifies target identifiers (usually, specific account IDs) to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources with the specified target IDs.</p>
    ///   - [`region_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::region_filters) / [`set_region_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_region_filters): <p>Specifies a list of Amazon Web Services Regions to limit the output to. If you use this parameter, the count of returned noncompliant resources includes only resources in the specified Regions.</p>
    ///   - [`resource_type_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::resource_type_filters) / [`set_resource_type_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_resource_type_filters): <p>Specifies that you want the response to include information for only resources of the specified types. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances.</p>  <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i> <a href="https://docs.aws.amazon.com/general/latest/gr/">Amazon Web Services General Reference</a> </i> for the following:</p>  <ul>   <li> <p>For a list of service name strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services Service Namespaces</a>.</p> </li>   <li> <p>For resource type strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li>   <li> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p> </li>  </ul>  <p>You can specify multiple resource types by using a comma separated array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    ///   - [`tag_key_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::tag_key_filters) / [`set_tag_key_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_tag_key_filters): <p>Specifies that you want the response to include information for only resources that have tags with the specified tag keys. If you use this parameter, the count of returned noncompliant resources includes only resources that have the specified tag keys.</p>
    ///   - [`group_by(Vec<GroupByAttribute>)`](crate::client::fluent_builders::GetComplianceSummary::group_by) / [`set_group_by(Option<Vec<GroupByAttribute>>)`](crate::client::fluent_builders::GetComplianceSummary::set_group_by): <p>Specifies a list of attributes to group the counts of noncompliant resources by. If supplied, the counts are sorted by those attributes.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetComplianceSummary::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetComplianceSummary::set_max_results): <p>Specifies the maximum number of results to be returned in each page. A query can return fewer than this maximum, even if there are more results still to return. You should always check the <code>PaginationToken</code> response value to see if there are more results. You can specify a minimum of 1 and a maximum value of 100.</p>
    ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::GetComplianceSummary::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::GetComplianceSummary::set_pagination_token): <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
                            /// - On success, responds with [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput) with field(s):
    ///   - [`summary_list(Option<Vec<Summary>>)`](crate::output::GetComplianceSummaryOutput::summary_list): <p>A table that shows counts of noncompliant resources.</p>
    ///   - [`pagination_token(Option<String>)`](crate::output::GetComplianceSummaryOutput::pagination_token): <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
                            /// - On failure, responds with [`SdkError<GetComplianceSummaryError>`](crate::error::GetComplianceSummaryError)
    pub fn get_compliance_summary(&self) -> crate::client::fluent_builders::GetComplianceSummary {
                                crate::client::fluent_builders::GetComplianceSummary::new(self.handle.clone())
                            }
}

