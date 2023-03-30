// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListQueues`](crate::client::fluent_builders::ListQueues) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListQueues::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`queue_name_prefix(impl Into<String>)`](crate::client::fluent_builders::ListQueues::queue_name_prefix) / [`set_queue_name_prefix(Option<String>)`](crate::client::fluent_builders::ListQueues::set_queue_name_prefix): <p>A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned.</p>  <p>Queue URLs and names are case-sensitive.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListQueues::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListQueues::set_next_token): <p>Pagination token to request the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListQueues::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListQueues::set_max_results): <p>Maximum number of results to include in the response. Value range is 1 to 1000. You must set <code>MaxResults</code> to receive a value for <code>NextToken</code> in the response.</p>
                            /// - On success, responds with [`ListQueuesOutput`](crate::output::ListQueuesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListQueuesOutput::next_token): <p>Pagination token to include in the next request. Token value is <code>null</code> if there are no additional results to request, or if you did not set <code>MaxResults</code> in the request.</p>
    ///   - [`queue_urls(Option<Vec<String>>)`](crate::output::ListQueuesOutput::queue_urls): <p>A list of queue URLs, up to 1,000 entries, or the value of MaxResults that you sent in the request.</p>
                            /// - On failure, responds with [`SdkError<ListQueuesError>`](crate::error::ListQueuesError)
    pub fn list_queues(&self) -> crate::client::fluent_builders::ListQueues {
                                crate::client::fluent_builders::ListQueues::new(self.handle.clone())
                            }
}

