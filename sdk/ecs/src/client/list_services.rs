// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListServices`](crate::client::fluent_builders::ListServices) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListServices::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::client::fluent_builders::ListServices::cluster) / [`set_cluster(Option<String>)`](crate::client::fluent_builders::ListServices::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListServices</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListServices::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListServices::set_next_token): <p>The <code>nextToken</code> value returned from a <code>ListServices</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note>   <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListServices::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListServices::set_max_results): <p>The maximum number of service results that <code>ListServices</code> returned in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`launch_type(LaunchType)`](crate::client::fluent_builders::ListServices::launch_type) / [`set_launch_type(Option<LaunchType>)`](crate::client::fluent_builders::ListServices::set_launch_type): <p>The launch type to use when filtering the <code>ListServices</code> results.</p>
    ///   - [`scheduling_strategy(SchedulingStrategy)`](crate::client::fluent_builders::ListServices::scheduling_strategy) / [`set_scheduling_strategy(Option<SchedulingStrategy>)`](crate::client::fluent_builders::ListServices::set_scheduling_strategy): <p>The scheduling strategy to use when filtering the <code>ListServices</code> results.</p>
                            /// - On success, responds with [`ListServicesOutput`](crate::output::ListServicesOutput) with field(s):
    ///   - [`service_arns(Option<Vec<String>>)`](crate::output::ListServicesOutput::service_arns): <p>The list of full ARN entries for each service that's associated with the specified cluster.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListServicesOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListServices</code> request. When the results of a <code>ListServices</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListServicesError>`](crate::error::ListServicesError)
    pub fn list_services(&self) -> crate::client::fluent_builders::ListServices {
                                crate::client::fluent_builders::ListServices::new(self.handle.clone())
                            }
}

