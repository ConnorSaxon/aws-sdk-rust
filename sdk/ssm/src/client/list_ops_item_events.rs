// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListOpsItemEvents`](crate::client::fluent_builders::ListOpsItemEvents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListOpsItemEvents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<OpsItemEventFilter>)`](crate::client::fluent_builders::ListOpsItemEvents::filters) / [`set_filters(Option<Vec<OpsItemEventFilter>>)`](crate::client::fluent_builders::ListOpsItemEvents::set_filters): <p>One or more OpsItem filters. Use a filter to return a more specific list of results. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListOpsItemEvents::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListOpsItemEvents::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListOpsItemEvents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListOpsItemEvents::set_next_token): <p>A token to start the list. Use this token to get the next set of results. </p>
                            /// - On success, responds with [`ListOpsItemEventsOutput`](crate::output::ListOpsItemEventsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListOpsItemEventsOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results. </p>
    ///   - [`summaries(Option<Vec<OpsItemEventSummary>>)`](crate::output::ListOpsItemEventsOutput::summaries): <p>A list of event information for the specified OpsItems.</p>
                            /// - On failure, responds with [`SdkError<ListOpsItemEventsError>`](crate::error::ListOpsItemEventsError)
    pub fn list_ops_item_events(&self) -> crate::client::fluent_builders::ListOpsItemEvents {
                                crate::client::fluent_builders::ListOpsItemEvents::new(self.handle.clone())
                            }
}

