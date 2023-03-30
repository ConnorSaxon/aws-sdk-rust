// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEvents`](crate::client::fluent_builders::DescribeEvents) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_name(impl Into<String>)`](crate::client::fluent_builders::DescribeEvents::source_name) / [`set_source_name(Option<String>)`](crate::client::fluent_builders::DescribeEvents::set_source_name): <p>The identifier of the event source for which events will be returned. If not specified, then all sources are included in the response.</p>
    ///   - [`source_type(SourceType)`](crate::client::fluent_builders::DescribeEvents::source_type) / [`set_source_type(Option<SourceType>)`](crate::client::fluent_builders::DescribeEvents::set_source_type): <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::DescribeEvents::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeEvents::set_start_time): <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::DescribeEvents::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeEvents::set_end_time): <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p>
    ///   - [`duration(i32)`](crate::client::fluent_builders::DescribeEvents::duration) / [`set_duration(Option<i32>)`](crate::client::fluent_builders::DescribeEvents::set_duration): <p>The number of minutes' worth of events to retrieve.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeEvents::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeEvents::set_max_results): <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>  <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeEvents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeEvents::set_next_token): <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
                            /// - On success, responds with [`DescribeEventsOutput`](crate::output::DescribeEventsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::DescribeEventsOutput::next_token): <p>Provides an identifier to allow retrieval of paginated results.</p>
    ///   - [`events(Option<Vec<Event>>)`](crate::output::DescribeEventsOutput::events): <p>An array of events. Each element in the array represents one event.</p>
                            /// - On failure, responds with [`SdkError<DescribeEventsError>`](crate::error::DescribeEventsError)
    pub fn describe_events(&self) -> crate::client::fluent_builders::DescribeEvents {
                                crate::client::fluent_builders::DescribeEvents::new(self.handle.clone())
                            }
}

