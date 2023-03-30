// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEndpointAccess`](crate::client::fluent_builders::ListEndpointAccess) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEndpointAccess::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEndpointAccess::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEndpointAccess::set_next_token): <p>If your initial <code>ListEndpointAccess</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in following <code>ListEndpointAccess</code> operations, which returns results in the next page.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEndpointAccess::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListEndpointAccess::set_max_results): <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to display the next page of results.</p>
    ///   - [`workgroup_name(impl Into<String>)`](crate::client::fluent_builders::ListEndpointAccess::workgroup_name) / [`set_workgroup_name(Option<String>)`](crate::client::fluent_builders::ListEndpointAccess::set_workgroup_name): <p>The name of the workgroup associated with the VPC endpoint to return.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::ListEndpointAccess::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::ListEndpointAccess::set_vpc_id): <p>The unique identifier of the virtual private cloud with access to Amazon Redshift Serverless.</p>
                            /// - On success, responds with [`ListEndpointAccessOutput`](crate::output::ListEndpointAccessOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListEndpointAccessOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    ///   - [`endpoints(Option<Vec<EndpointAccess>>)`](crate::output::ListEndpointAccessOutput::endpoints): <p>The returned VPC endpoints.</p>
                            /// - On failure, responds with [`SdkError<ListEndpointAccessError>`](crate::error::ListEndpointAccessError)
    pub fn list_endpoint_access(&self) -> crate::client::fluent_builders::ListEndpointAccess {
                                crate::client::fluent_builders::ListEndpointAccess::new(self.handle.clone())
                            }
}

