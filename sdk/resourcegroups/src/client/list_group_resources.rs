// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListGroupResources`](crate::client::fluent_builders::ListGroupResources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListGroupResources::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::client::fluent_builders::ListGroupResources::group_name) / [`set_group_name(Option<String>)`](crate::client::fluent_builders::ListGroupResources::set_group_name): <important>   <p> <i> <b>Deprecated - don't use this parameter. Use the <code>Group</code> request field instead.</b> </i> </p>  </important>
    ///   - [`group(impl Into<String>)`](crate::client::fluent_builders::ListGroupResources::group) / [`set_group(Option<String>)`](crate::client::fluent_builders::ListGroupResources::set_group): <p>The name or the ARN of the resource group</p>
    ///   - [`filters(Vec<ResourceFilter>)`](crate::client::fluent_builders::ListGroupResources::filters) / [`set_filters(Option<Vec<ResourceFilter>>)`](crate::client::fluent_builders::ListGroupResources::set_filters): <p>Filters, formatted as <code>ResourceFilter</code> objects, that you want to apply to a <code>ListGroupResources</code> operation. Filters the results to include only those of the specified resource types.</p>  <ul>   <li> <p> <code>resource-type</code> - Filter resources by their type. Specify up to five resource types in the format <code>AWS::ServiceCode::ResourceType</code>. For example, <code>AWS::EC2::Instance</code>, or <code>AWS::S3::Bucket</code>. </p> </li>  </ul>  <p>When you specify a <code>resource-type</code> filter for <code>ListGroupResources</code>, Resource Groups validates your filter resource types against the types that are defined in the query associated with the group. For example, if a group contains only S3 buckets because its query specifies only that resource type, but your <code>resource-type</code> filter includes EC2 instances, AWS Resource Groups does not filter for EC2 instances. In this case, a <code>ListGroupResources</code> request returns a <code>BadRequestException</code> error with a message similar to the following:</p>  <p> <code>The resource types specified as filters in the request are not valid.</code> </p>  <p>The error includes a list of resource types that failed the validation because they are not part of the query associated with the group. This validation doesn't occur when the group query specifies <code>AWS::AllSupported</code>, because a group based on such a query can contain any of the allowed resource types for the query type (tag-based or Amazon CloudFront stack-based queries).</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListGroupResources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListGroupResources::set_max_results): <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListGroupResources::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListGroupResources::set_next_token): <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
                            /// - On success, responds with [`ListGroupResourcesOutput`](crate::output::ListGroupResourcesOutput) with field(s):
    ///   - [`resources(Option<Vec<ListGroupResourcesItem>>)`](crate::output::ListGroupResourcesOutput::resources): <p>An array of resources from which you can determine each resource's identity, type, and group membership status.</p>
    ///   - [`resource_identifiers(Option<Vec<ResourceIdentifier>>)`](crate::output::ListGroupResourcesOutput::resource_identifiers): <important>   <p> <b> <i>Deprecated - don't use this parameter. Use the <code>Resources</code> response field instead.</i> </b> </p>  </important>
    ///   - [`next_token(Option<String>)`](crate::output::ListGroupResourcesOutput::next_token): <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    ///   - [`query_errors(Option<Vec<QueryError>>)`](crate::output::ListGroupResourcesOutput::query_errors): <p>A list of <code>QueryError</code> objects. Each error is an object that contains <code>ErrorCode</code> and <code>Message</code> structures. Possible values for <code>ErrorCode</code> are <code>CLOUDFORMATION_STACK_INACTIVE</code> and <code>CLOUDFORMATION_STACK_NOT_EXISTING</code>.</p>
                            /// - On failure, responds with [`SdkError<ListGroupResourcesError>`](crate::error::ListGroupResourcesError)
    pub fn list_group_resources(&self) -> crate::client::fluent_builders::ListGroupResources {
                                crate::client::fluent_builders::ListGroupResources::new(self.handle.clone())
                            }
}

