// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListNetworkResources`](crate::client::fluent_builders::ListNetworkResources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListNetworkResources::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(HashMap<NetworkResourceFilterKeys, Vec<String>>)`](crate::client::fluent_builders::ListNetworkResources::filters) / [`set_filters(Option<HashMap<NetworkResourceFilterKeys, Vec<String>>>)`](crate::client::fluent_builders::ListNetworkResources::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>ORDER</code> - The Amazon Resource Name (ARN) of the order.</p> </li>   <li> <p> <code>STATUS</code> - The status (<code>AVAILABLE</code> | <code>DELETED</code> | <code>DELETING</code> | <code>PENDING</code> | <code>PENDING_RETURN</code> | <code>PROVISIONING</code> | <code>SHIPPED</code>).</p> </li>  </ul>  <p>Filter values are case sensitive. If you specify multiple values for a filter, the values are joined with an <code>OR</code>, and the request returns all results that match any of the specified values.</p>
    ///   - [`network_arn(impl Into<String>)`](crate::client::fluent_builders::ListNetworkResources::network_arn) / [`set_network_arn(Option<String>)`](crate::client::fluent_builders::ListNetworkResources::set_network_arn): <p>The Amazon Resource Name (ARN) of the network.</p>
    ///   - [`start_token(impl Into<String>)`](crate::client::fluent_builders::ListNetworkResources::start_token) / [`set_start_token(Option<String>)`](crate::client::fluent_builders::ListNetworkResources::set_start_token): <p>The token for the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListNetworkResources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListNetworkResources::set_max_results): <p>The maximum number of results to return.</p>
                            /// - On success, responds with [`ListNetworkResourcesOutput`](crate::output::ListNetworkResourcesOutput) with field(s):
    ///   - [`network_resources(Option<Vec<NetworkResource>>)`](crate::output::ListNetworkResourcesOutput::network_resources): <p>Information about network resources.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListNetworkResourcesOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListNetworkResourcesError>`](crate::error::ListNetworkResourcesError)
    pub fn list_network_resources(&self) -> crate::client::fluent_builders::ListNetworkResources {
                                crate::client::fluent_builders::ListNetworkResources::new(self.handle.clone())
                            }
}

