// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeploymentJobs`](crate::client::fluent_builders::ListDeploymentJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDeploymentJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListDeploymentJobs::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListDeploymentJobs::set_filters): <p>Optional filters to limit results.</p>  <p>The filter names <code>status</code> and <code>fleetName</code> are supported. When filtering, you must use the complete value of the filtered item. You can use up to three filters, but they must be for the same named item. For example, if you are looking for items with the status <code>InProgress</code> or the status <code>Pending</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDeploymentJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDeploymentJobs::set_next_token): <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListDeploymentJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDeploymentJobs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDeploymentJobs::set_max_results): <p>When this parameter is used, <code>ListDeploymentJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListDeploymentJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 200. If this parameter is not used, then <code>ListDeploymentJobs</code> returns up to 200 results and a <code>nextToken</code> value if applicable. </p>
                            /// - On success, responds with [`ListDeploymentJobsOutput`](crate::output::ListDeploymentJobsOutput) with field(s):
    ///   - [`deployment_jobs(Option<Vec<DeploymentJob>>)`](crate::output::ListDeploymentJobsOutput::deployment_jobs): <p>A list of deployment jobs that meet the criteria of the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDeploymentJobsOutput::next_token): <p>If the previous paginated request did not return all of the remaining results, the response object's <code>nextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListDeploymentJobs</code> again and assign that token to the request object's <code>nextToken</code> parameter. If there are no remaining results, the previous response object's NextToken parameter is set to null. </p>
                            /// - On failure, responds with [`SdkError<ListDeploymentJobsError>`](crate::error::ListDeploymentJobsError)
    #[deprecated(note = "Support for the AWS RoboMaker application deployment feature has ended. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/fleets.html.")]
    pub fn list_deployment_jobs(&self) -> crate::client::fluent_builders::ListDeploymentJobs {
                                crate::client::fluent_builders::ListDeploymentJobs::new(self.handle.clone())
                            }
}

