// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeployments`](crate::client::fluent_builders::ListDeployments) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDeployments::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`target_arn(impl Into<String>)`](crate::client::fluent_builders::ListDeployments::target_arn) / [`set_target_arn(Option<String>)`](crate::client::fluent_builders::ListDeployments::set_target_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group.</p>
    ///   - [`history_filter(DeploymentHistoryFilter)`](crate::client::fluent_builders::ListDeployments::history_filter) / [`set_history_filter(Option<DeploymentHistoryFilter>)`](crate::client::fluent_builders::ListDeployments::set_history_filter): <p>The filter for the list of deployments. Choose one of the following options:</p>  <ul>   <li> <p> <code>ALL</code> – The list includes all deployments.</p> </li>   <li> <p> <code>LATEST_ONLY</code> – The list includes only the latest revision of each deployment.</p> </li>  </ul>  <p>Default: <code>LATEST_ONLY</code> </p>
    ///   - [`parent_target_arn(impl Into<String>)`](crate::client::fluent_builders::ListDeployments::parent_target_arn) / [`set_parent_target_arn(Option<String>)`](crate::client::fluent_builders::ListDeployments::set_parent_target_arn): <p>The parent deployment's target <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> within a subdeployment.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDeployments::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDeployments::set_max_results): <p>The maximum number of results to be returned per paginated request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDeployments::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDeployments::set_next_token): <p>The token to be used for the next set of paginated results.</p>
                            /// - On success, responds with [`ListDeploymentsOutput`](crate::output::ListDeploymentsOutput) with field(s):
    ///   - [`deployments(Option<Vec<Deployment>>)`](crate::output::ListDeploymentsOutput::deployments): <p>A list that summarizes each deployment.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDeploymentsOutput::next_token): <p>The token for the next set of results, or null if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListDeploymentsError>`](crate::error::ListDeploymentsError)
    pub fn list_deployments(&self) -> crate::client::fluent_builders::ListDeployments {
                                crate::client::fluent_builders::ListDeployments::new(self.handle.clone())
                            }
}

