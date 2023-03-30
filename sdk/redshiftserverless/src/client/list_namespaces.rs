// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListNamespaces`](crate::client::fluent_builders::ListNamespaces) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListNamespaces::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListNamespaces::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListNamespaces::set_next_token): <p>If your initial <code>ListNamespaces</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in following <code>ListNamespaces</code> operations, which returns results in the next page.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListNamespaces::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListNamespaces::set_max_results): <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to display the next page of results.</p>
                            /// - On success, responds with [`ListNamespacesOutput`](crate::output::ListNamespacesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListNamespacesOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    ///   - [`namespaces(Option<Vec<Namespace>>)`](crate::output::ListNamespacesOutput::namespaces): <p>The list of returned namespaces.</p>
                            /// - On failure, responds with [`SdkError<ListNamespacesError>`](crate::error::ListNamespacesError)
    pub fn list_namespaces(&self) -> crate::client::fluent_builders::ListNamespaces {
                                crate::client::fluent_builders::ListNamespaces::new(self.handle.clone())
                            }
}

