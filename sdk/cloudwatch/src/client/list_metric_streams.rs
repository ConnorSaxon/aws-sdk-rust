// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListMetricStreams`](crate::client::fluent_builders::ListMetricStreams) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListMetricStreams::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListMetricStreams::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListMetricStreams::set_next_token): <p>Include this value, if it was returned by the previous call, to get the next set of metric streams.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListMetricStreams::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListMetricStreams::set_max_results): <p>The maximum number of results to return in one operation.</p>
                            /// - On success, responds with [`ListMetricStreamsOutput`](crate::output::ListMetricStreamsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListMetricStreamsOutput::next_token): <p>The token that marks the start of the next batch of returned results. You can use this token in a subsequent operation to get the next batch of results.</p>
    ///   - [`entries(Option<Vec<MetricStreamEntry>>)`](crate::output::ListMetricStreamsOutput::entries): <p>The array of metric stream information.</p>
                            /// - On failure, responds with [`SdkError<ListMetricStreamsError>`](crate::error::ListMetricStreamsError)
    pub fn list_metric_streams(&self) -> crate::client::fluent_builders::ListMetricStreams {
                                crate::client::fluent_builders::ListMetricStreams::new(self.handle.clone())
                            }
}

