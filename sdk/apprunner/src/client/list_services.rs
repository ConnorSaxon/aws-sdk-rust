// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListServices`](crate::client::fluent_builders::ListServices) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListServices::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListServices::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListServices::set_next_token): <p>A token from a previous result page. Used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones specified in the initial request.</p>  <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListServices::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListServices::set_max_results): <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>  <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
                            /// - On success, responds with [`ListServicesOutput`](crate::output::ListServicesOutput) with field(s):
    ///   - [`service_summary_list(Option<Vec<ServiceSummary>>)`](crate::output::ListServicesOutput::service_summary_list): <p>A list of service summary information records. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListServicesOutput::next_token): <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
                            /// - On failure, responds with [`SdkError<ListServicesError>`](crate::error::ListServicesError)
    pub fn list_services(&self) -> crate::client::fluent_builders::ListServices {
                                crate::client::fluent_builders::ListServices::new(self.handle.clone())
                            }
}

