// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAccessPolicy`](crate::client::fluent_builders::DescribeAccessPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`access_policy_id(impl Into<String>)`](crate::client::fluent_builders::DescribeAccessPolicy::access_policy_id) / [`set_access_policy_id(Option<String>)`](crate::client::fluent_builders::DescribeAccessPolicy::set_access_policy_id): <p>The ID of the access policy.</p>
                            /// - On success, responds with [`DescribeAccessPolicyOutput`](crate::output::DescribeAccessPolicyOutput) with field(s):
    ///   - [`access_policy_id(Option<String>)`](crate::output::DescribeAccessPolicyOutput::access_policy_id): <p>The ID of the access policy.</p>
    ///   - [`access_policy_arn(Option<String>)`](crate::output::DescribeAccessPolicyOutput::access_policy_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the access policy, which has the following format.</p>  <p> <code>arn:${Partition}:iotsitewise:${Region}:${Account}:access-policy/${AccessPolicyId}</code> </p>
    ///   - [`access_policy_identity(Option<Identity>)`](crate::output::DescribeAccessPolicyOutput::access_policy_identity): <p>The identity (IAM Identity Center user, IAM Identity Center group, or IAM user) to which this access policy applies.</p>
    ///   - [`access_policy_resource(Option<Resource>)`](crate::output::DescribeAccessPolicyOutput::access_policy_resource): <p>The IoT SiteWise Monitor resource (portal or project) to which this access policy provides access.</p>
    ///   - [`access_policy_permission(Option<Permission>)`](crate::output::DescribeAccessPolicyOutput::access_policy_permission): <p>The access policy permission. Note that a project <code>ADMINISTRATOR</code> is also known as a project owner.</p>
    ///   - [`access_policy_creation_date(Option<DateTime>)`](crate::output::DescribeAccessPolicyOutput::access_policy_creation_date): <p>The date the access policy was created, in Unix epoch time.</p>
    ///   - [`access_policy_last_update_date(Option<DateTime>)`](crate::output::DescribeAccessPolicyOutput::access_policy_last_update_date): <p>The date the access policy was last updated, in Unix epoch time.</p>
                            /// - On failure, responds with [`SdkError<DescribeAccessPolicyError>`](crate::error::DescribeAccessPolicyError)
    pub fn describe_access_policy(&self) -> crate::client::fluent_builders::DescribeAccessPolicy {
                                crate::client::fluent_builders::DescribeAccessPolicy::new(self.handle.clone())
                            }
}

