// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListContexts`](crate::client::fluent_builders::ListContexts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListContexts::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_uri(impl Into<String>)`](crate::client::fluent_builders::ListContexts::source_uri) / [`set_source_uri(Option<String>)`](crate::client::fluent_builders::ListContexts::set_source_uri): <p>A filter that returns only contexts with the specified source URI.</p>
    ///   - [`context_type(impl Into<String>)`](crate::client::fluent_builders::ListContexts::context_type) / [`set_context_type(Option<String>)`](crate::client::fluent_builders::ListContexts::set_context_type): <p>A filter that returns only contexts of the specified type.</p>
    ///   - [`created_after(DateTime)`](crate::client::fluent_builders::ListContexts::created_after) / [`set_created_after(Option<DateTime>)`](crate::client::fluent_builders::ListContexts::set_created_after): <p>A filter that returns only contexts created on or after the specified time.</p>
    ///   - [`created_before(DateTime)`](crate::client::fluent_builders::ListContexts::created_before) / [`set_created_before(Option<DateTime>)`](crate::client::fluent_builders::ListContexts::set_created_before): <p>A filter that returns only contexts created on or before the specified time.</p>
    ///   - [`sort_by(SortContextsBy)`](crate::client::fluent_builders::ListContexts::sort_by) / [`set_sort_by(Option<SortContextsBy>)`](crate::client::fluent_builders::ListContexts::set_sort_by): <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    ///   - [`sort_order(SortOrder)`](crate::client::fluent_builders::ListContexts::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::client::fluent_builders::ListContexts::set_sort_order): <p>The sort order. The default value is <code>Descending</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListContexts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListContexts::set_next_token): <p>If the previous call to <code>ListContexts</code> didn't return the full set of contexts, the call returns a token for getting the next set of contexts.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListContexts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListContexts::set_max_results): <p>The maximum number of contexts to return in the response. The default value is 10.</p>
                            /// - On success, responds with [`ListContextsOutput`](crate::output::ListContextsOutput) with field(s):
    ///   - [`context_summaries(Option<Vec<ContextSummary>>)`](crate::output::ListContextsOutput::context_summaries): <p>A list of contexts and their properties.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListContextsOutput::next_token): <p>A token for getting the next set of contexts, if there are any.</p>
                            /// - On failure, responds with [`SdkError<ListContextsError>`](crate::error::ListContextsError)
    pub fn list_contexts(&self) -> crate::client::fluent_builders::ListContexts {
                                crate::client::fluent_builders::ListContexts::new(self.handle.clone())
                            }
}

