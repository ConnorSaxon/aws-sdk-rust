// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListVpcConnectors`](crate::client::fluent_builders::ListVpcConnectors) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListVpcConnectors::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListVpcConnectors::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListVpcConnectors::set_max_results): <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>  <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListVpcConnectors::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListVpcConnectors::set_next_token): <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones that are specified in the initial request.</p>  <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
                            /// - On success, responds with [`ListVpcConnectorsOutput`](crate::output::ListVpcConnectorsOutput) with field(s):
    ///   - [`vpc_connectors(Option<Vec<VpcConnector>>)`](crate::output::ListVpcConnectorsOutput::vpc_connectors): <p>A list of information records for VPC connectors. In a paginated request, the request returns up to <code>MaxResults</code> records for each call.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListVpcConnectorsOutput::next_token): <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p>
                            /// - On failure, responds with [`SdkError<ListVpcConnectorsError>`](crate::error::ListVpcConnectorsError)
    pub fn list_vpc_connectors(&self) -> crate::client::fluent_builders::ListVpcConnectors {
                                crate::client::fluent_builders::ListVpcConnectors::new(self.handle.clone())
                            }
}

