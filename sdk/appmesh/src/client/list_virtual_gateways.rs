// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListVirtualGateways`](crate::client::fluent_builders::ListVirtualGateways) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListVirtualGateways::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`mesh_name(impl Into<String>)`](crate::client::fluent_builders::ListVirtualGateways::mesh_name) / [`set_mesh_name(Option<String>)`](crate::client::fluent_builders::ListVirtualGateways::set_mesh_name): <p>The name of the service mesh to list virtual gateways in.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListVirtualGateways::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListVirtualGateways::set_next_token): <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualGateways</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListVirtualGateways::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListVirtualGateways::set_limit): <p>The maximum number of results returned by <code>ListVirtualGateways</code> in paginated output. When you use this parameter, <code>ListVirtualGateways</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualGateways</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualGateways</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`mesh_owner(impl Into<String>)`](crate::client::fluent_builders::ListVirtualGateways::mesh_owner) / [`set_mesh_owner(Option<String>)`](crate::client::fluent_builders::ListVirtualGateways::set_mesh_owner): <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
                            /// - On success, responds with [`ListVirtualGatewaysOutput`](crate::output::ListVirtualGatewaysOutput) with field(s):
    ///   - [`virtual_gateways(Option<Vec<VirtualGatewayRef>>)`](crate::output::ListVirtualGatewaysOutput::virtual_gateways): <p>The list of existing virtual gateways for the specified service mesh.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListVirtualGatewaysOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListVirtualGateways</code> request. When the results of a <code>ListVirtualGateways</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListVirtualGatewaysError>`](crate::error::ListVirtualGatewaysError)
    pub fn list_virtual_gateways(&self) -> crate::client::fluent_builders::ListVirtualGateways {
                                crate::client::fluent_builders::ListVirtualGateways::new(self.handle.clone())
                            }
}

