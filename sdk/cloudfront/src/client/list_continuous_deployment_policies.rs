// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListContinuousDeploymentPolicies`](crate::client::fluent_builders::ListContinuousDeploymentPolicies) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListContinuousDeploymentPolicies::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListContinuousDeploymentPolicies::set_marker): <p>Use this field when paginating results to indicate where to begin in your list of continuous deployment policies. The response includes policies in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListContinuousDeploymentPolicies::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListContinuousDeploymentPolicies::set_max_items): <p>The maximum number of continuous deployment policies that you want returned in the response.</p>
                            /// - On success, responds with [`ListContinuousDeploymentPoliciesOutput`](crate::output::ListContinuousDeploymentPoliciesOutput) with field(s):
    ///   - [`continuous_deployment_policy_list(Option<ContinuousDeploymentPolicyList>)`](crate::output::ListContinuousDeploymentPoliciesOutput::continuous_deployment_policy_list): <p>A list of continuous deployment policies.</p>
                            /// - On failure, responds with [`SdkError<ListContinuousDeploymentPoliciesError>`](crate::error::ListContinuousDeploymentPoliciesError)
    pub fn list_continuous_deployment_policies(&self) -> crate::client::fluent_builders::ListContinuousDeploymentPolicies {
                                crate::client::fluent_builders::ListContinuousDeploymentPolicies::new(self.handle.clone())
                            }
}

