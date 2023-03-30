// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListManagedResources`](crate::client::fluent_builders::ListManagedResources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListManagedResources::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListManagedResources::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListManagedResources::set_next_token): <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListManagedResources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListManagedResources::set_max_results): <p>The number of objects that you want to return with this call.</p>
                            /// - On success, responds with [`ListManagedResourcesOutput`](crate::output::ListManagedResourcesOutput) with field(s):
    ///   - [`items(Option<Vec<ManagedResourceSummary>>)`](crate::output::ListManagedResourcesOutput::items): <p>The items in the response list.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListManagedResourcesOutput::next_token): <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListManagedResourcesError>`](crate::error::ListManagedResourcesError)
    pub fn list_managed_resources(&self) -> crate::client::fluent_builders::ListManagedResources {
                                crate::client::fluent_builders::ListManagedResources::new(self.handle.clone())
                            }
}

