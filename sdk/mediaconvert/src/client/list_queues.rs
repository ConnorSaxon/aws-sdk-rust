// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListQueues`](crate::client::fluent_builders::ListQueues) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListQueues::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`list_by(QueueListBy)`](crate::client::fluent_builders::ListQueues::list_by) / [`set_list_by(Option<QueueListBy>)`](crate::client::fluent_builders::ListQueues::set_list_by): Optional. When you request a list of queues, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don't specify, the service will list them by creation date.
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListQueues::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListQueues::set_max_results): Optional. Number of queues, up to twenty, that will be returned at one time.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListQueues::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListQueues::set_next_token): Use this string, provided with the response to a previous request, to request the next batch of queues.
    ///   - [`order(Order)`](crate::client::fluent_builders::ListQueues::order) / [`set_order(Option<Order>)`](crate::client::fluent_builders::ListQueues::set_order): Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.
                            /// - On success, responds with [`ListQueuesOutput`](crate::output::ListQueuesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListQueuesOutput::next_token): Use this string to request the next batch of queues.
    ///   - [`queues(Option<Vec<Queue>>)`](crate::output::ListQueuesOutput::queues): List of queues.
                            /// - On failure, responds with [`SdkError<ListQueuesError>`](crate::error::ListQueuesError)
    pub fn list_queues(&self) -> crate::client::fluent_builders::ListQueues {
                                crate::client::fluent_builders::ListQueues::new(self.handle.clone())
                            }
}

