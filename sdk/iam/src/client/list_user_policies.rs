// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListUserPolicies`](crate::client::fluent_builders::ListUserPolicies) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUserPolicies::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::ListUserPolicies::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::ListUserPolicies::set_user_name): <p>The name of the user to list policies for.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListUserPolicies::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListUserPolicies::set_marker): <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListUserPolicies::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListUserPolicies::set_max_items): <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>  <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
                            /// - On success, responds with [`ListUserPoliciesOutput`](crate::output::ListUserPoliciesOutput) with field(s):
    ///   - [`policy_names(Option<Vec<String>>)`](crate::output::ListUserPoliciesOutput::policy_names): <p>A list of policy names.</p>
    ///   - [`is_truncated(bool)`](crate::output::ListUserPoliciesOutput::is_truncated): <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    ///   - [`marker(Option<String>)`](crate::output::ListUserPoliciesOutput::marker): <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
                            /// - On failure, responds with [`SdkError<ListUserPoliciesError>`](crate::error::ListUserPoliciesError)
    pub fn list_user_policies(&self) -> crate::client::fluent_builders::ListUserPolicies {
                                crate::client::fluent_builders::ListUserPolicies::new(self.handle.clone())
                            }
}

