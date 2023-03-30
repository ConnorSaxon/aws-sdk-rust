// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPolicyVersions`](crate::client::fluent_builders::ListPolicyVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPolicyVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::client::fluent_builders::ListPolicyVersions::policy_arn) / [`set_policy_arn(Option<String>)`](crate::client::fluent_builders::ListPolicyVersions::set_policy_arn): <p>The Amazon Resource Name (ARN) of the IAM policy for which you want the versions.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListPolicyVersions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListPolicyVersions::set_marker): <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListPolicyVersions::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListPolicyVersions::set_max_items): <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>  <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
                            /// - On success, responds with [`ListPolicyVersionsOutput`](crate::output::ListPolicyVersionsOutput) with field(s):
    ///   - [`versions(Option<Vec<PolicyVersion>>)`](crate::output::ListPolicyVersionsOutput::versions): <p>A list of policy versions.</p>  <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    ///   - [`is_truncated(bool)`](crate::output::ListPolicyVersionsOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    ///   - [`marker(Option<String>)`](crate::output::ListPolicyVersionsOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
                            /// - On failure, responds with [`SdkError<ListPolicyVersionsError>`](crate::error::ListPolicyVersionsError)
    pub fn list_policy_versions(&self) -> crate::client::fluent_builders::ListPolicyVersions {
                                crate::client::fluent_builders::ListPolicyVersions::new(self.handle.clone())
                            }
}

