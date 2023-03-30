// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEntitiesForPolicy`](crate::client::fluent_builders::ListEntitiesForPolicy) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEntitiesForPolicy::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::policy_arn) / [`set_policy_arn(Option<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_policy_arn): <p>The Amazon Resource Name (ARN) of the IAM policy for which you want the versions.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`entity_filter(EntityType)`](crate::client::fluent_builders::ListEntitiesForPolicy::entity_filter) / [`set_entity_filter(Option<EntityType>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_entity_filter): <p>The entity type to use for filtering the results.</p>  <p>For example, when <code>EntityFilter</code> is <code>Role</code>, only the roles that are attached to the specified policy are returned. This parameter is optional. If it is not included, all attached entities (users, groups, and roles) are returned. The argument for this parameter must be one of the valid values listed below.</p>
    ///   - [`path_prefix(impl Into<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::path_prefix) / [`set_path_prefix(Option<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_path_prefix): <p>The path prefix for filtering the results. This parameter is optional. If it is not included, it defaults to a slash (/), listing all entities.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p>
    ///   - [`policy_usage_filter(PolicyUsageType)`](crate::client::fluent_builders::ListEntitiesForPolicy::policy_usage_filter) / [`set_policy_usage_filter(Option<PolicyUsageType>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_policy_usage_filter): <p>The policy usage method to use for filtering the results.</p>  <p>To list only permissions policies, set&nbsp;<code>PolicyUsageFilter</code>&nbsp;to&nbsp;<code>PermissionsPolicy</code>. To list only the policies used to set permissions boundaries, set&nbsp;the value to&nbsp;<code>PermissionsBoundary</code>.</p>  <p>This parameter is optional. If it is not included, all policies are returned. </p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_marker): <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListEntitiesForPolicy::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListEntitiesForPolicy::set_max_items): <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>  <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
                            /// - On success, responds with [`ListEntitiesForPolicyOutput`](crate::output::ListEntitiesForPolicyOutput) with field(s):
    ///   - [`policy_groups(Option<Vec<PolicyGroup>>)`](crate::output::ListEntitiesForPolicyOutput::policy_groups): <p>A list of IAM groups that the policy is attached to.</p>
    ///   - [`policy_users(Option<Vec<PolicyUser>>)`](crate::output::ListEntitiesForPolicyOutput::policy_users): <p>A list of IAM users that the policy is attached to.</p>
    ///   - [`policy_roles(Option<Vec<PolicyRole>>)`](crate::output::ListEntitiesForPolicyOutput::policy_roles): <p>A list of IAM roles that the policy is attached to.</p>
    ///   - [`is_truncated(bool)`](crate::output::ListEntitiesForPolicyOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    ///   - [`marker(Option<String>)`](crate::output::ListEntitiesForPolicyOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
                            /// - On failure, responds with [`SdkError<ListEntitiesForPolicyError>`](crate::error::ListEntitiesForPolicyError)
    pub fn list_entities_for_policy(&self) -> crate::client::fluent_builders::ListEntitiesForPolicy {
                                crate::client::fluent_builders::ListEntitiesForPolicy::new(self.handle.clone())
                            }
}

