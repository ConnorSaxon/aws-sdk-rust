// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListNodes`](crate::client::fluent_builders::ListNodes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListNodes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl Into<String>)`](crate::client::fluent_builders::ListNodes::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::client::fluent_builders::ListNodes::set_cluster_arn): <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListNodes::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListNodes::set_max_results): <p>The maximum number of results to return in the response. If there are more results, the response includes a NextToken parameter.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListNodes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListNodes::set_next_token): <p>The paginated results marker. When the result of the operation is truncated, the call returns NextToken in the response. To get the next batch, provide this token in your next request.</p>
                            /// - On success, responds with [`ListNodesOutput`](crate::output::ListNodesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListNodesOutput::next_token): <p>The paginated results marker. When the result of a ListNodes operation is truncated, the call returns NextToken in the response. To get another batch of nodes, provide this token in your next request.</p>
    ///   - [`node_info_list(Option<Vec<NodeInfo>>)`](crate::output::ListNodesOutput::node_info_list): <p>List containing a NodeInfo object.</p>
                            /// - On failure, responds with [`SdkError<ListNodesError>`](crate::error::ListNodesError)
    pub fn list_nodes(&self) -> crate::client::fluent_builders::ListNodes {
                                crate::client::fluent_builders::ListNodes::new(self.handle.clone())
                            }
}

