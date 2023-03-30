// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOrganizationResourceCollectionHealth`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_resource_collection_type(OrganizationResourceCollectionType)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::organization_resource_collection_type) / [`set_organization_resource_collection_type(Option<OrganizationResourceCollectionType>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::set_organization_resource_collection_type): <p> An Amazon Web Services resource collection type. This type specifies how analyzed Amazon Web Services resources are defined. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. DevOps Guru can be configured to analyze the Amazon Web Services resources that are defined in the stacks or that are tagged using the same tag <i>key</i>. You can specify up to 500 Amazon Web Services CloudFormation stacks. </p>
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::set_account_ids): <p>The ID of the Amazon Web Services account.</p>
    ///   - [`organizational_unit_ids(Vec<String>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::organizational_unit_ids) / [`set_organizational_unit_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::set_organizational_unit_ids): <p>The ID of the organizational unit.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::set_next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
                            /// - On success, responds with [`DescribeOrganizationResourceCollectionHealthOutput`](crate::output::DescribeOrganizationResourceCollectionHealthOutput) with field(s):
    ///   - [`cloud_formation(Option<Vec<CloudFormationHealth>>)`](crate::output::DescribeOrganizationResourceCollectionHealthOutput::cloud_formation): <p>The returned <code>CloudFormationHealthOverview</code> object that contains an <code>InsightHealthOverview</code> object with the requested system health information.</p>
    ///   - [`service(Option<Vec<ServiceHealth>>)`](crate::output::DescribeOrganizationResourceCollectionHealthOutput::service): <p>An array of <code>ServiceHealth</code> objects that describes the health of the Amazon Web Services services associated with the resources in the collection.</p>
    ///   - [`account(Option<Vec<AccountHealth>>)`](crate::output::DescribeOrganizationResourceCollectionHealthOutput::account): <p>The name of the organization's account.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeOrganizationResourceCollectionHealthOutput::next_token): <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    ///   - [`tags(Option<Vec<TagHealth>>)`](crate::output::DescribeOrganizationResourceCollectionHealthOutput::tags): <p>Tags help you identify and organize your Amazon Web Services resources. Many Amazon Web Services services support tagging, so you can assign the same tag to resources from different services to indicate that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB table resource that you assign to an Lambda function. For more information about using tags, see the <a href="https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf">Tagging best practices</a> whitepaper. </p>  <p>Each Amazon Web Services tag has two parts. </p>  <ul>   <li> <p>A tag <i>key</i> (for example, <code>CostCenter</code>, <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag <i>keys</i> are case-sensitive.</p> </li>   <li> <p>An optional field known as a tag <i>value</i> (for example, <code>111122223333</code>, <code>Production</code>, or a team name). Omitting the tag <i>value</i> is the same as using an empty string. Like tag <i>keys</i>, tag <i>values</i> are case-sensitive.</p> </li>  </ul>  <p>Together these are known as <i>key</i>-<i>value</i> pairs.</p> <important>   <p>The string used for a <i>key</i> in a tag that you use to define your resource coverage must begin with the prefix <code>Devops-guru-</code>. The tag <i>key</i> might be <code>DevOps-Guru-deployment-application</code> or <code>devops-guru-rds-application</code>. When you create a <i>key</i>, the case of characters in the <i>key</i> can be whatever you choose. After you create a <i>key</i>, it is case-sensitive. For example, DevOps Guru works with a <i>key</i> named <code>devops-guru-rds</code> and a <i>key</i> named <code>DevOps-Guru-RDS</code>, and these act as two different <i>keys</i>. Possible <i>key</i>/<i>value</i> pairs in your application might be <code>Devops-Guru-production-application/RDS</code> or <code>Devops-Guru-production-application/containers</code>.</p>  </important>
                            /// - On failure, responds with [`SdkError<DescribeOrganizationResourceCollectionHealthError>`](crate::error::DescribeOrganizationResourceCollectionHealthError)
    pub fn describe_organization_resource_collection_health(&self) -> crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth {
                                crate::client::fluent_builders::DescribeOrganizationResourceCollectionHealth::new(self.handle.clone())
                            }
}

