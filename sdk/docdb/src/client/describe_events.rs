// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEvents`](crate::client::fluent_builders::DescribeEvents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeEvents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeEvents::source_identifier) / [`set_source_identifier(Option<String>)`](crate::client::fluent_builders::DescribeEvents::set_source_identifier): <p>The identifier of the event source for which events are returned. If not specified, then all sources are included in the response.</p>  <p>Constraints:</p>  <ul>   <li> <p>If <code>SourceIdentifier</code> is provided, <code>SourceType</code> must also be provided.</p> </li>   <li> <p>If the source type is <code>DBInstance</code>, a <code>DBInstanceIdentifier</code> must be provided.</p> </li>   <li> <p>If the source type is <code>DBSecurityGroup</code>, a <code>DBSecurityGroupName</code> must be provided.</p> </li>   <li> <p>If the source type is <code>DBParameterGroup</code>, a <code>DBParameterGroupName</code> must be provided.</p> </li>   <li> <p>If the source type is <code>DBSnapshot</code>, a <code>DBSnapshotIdentifier</code> must be provided.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul>
    ///   - [`source_type(SourceType)`](crate::client::fluent_builders::DescribeEvents::source_type) / [`set_source_type(Option<SourceType>)`](crate::client::fluent_builders::DescribeEvents::set_source_type): <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::DescribeEvents::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeEvents::set_start_time): <p> The beginning of the time interval to retrieve events for, specified in ISO 8601 format. </p>  <p>Example: 2009-07-08T18:00Z</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::DescribeEvents::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeEvents::set_end_time): <p> The end of the time interval for which to retrieve events, specified in ISO 8601 format. </p>  <p>Example: 2009-07-08T18:00Z</p>
    ///   - [`duration(i32)`](crate::client::fluent_builders::DescribeEvents::duration) / [`set_duration(Option<i32>)`](crate::client::fluent_builders::DescribeEvents::set_duration): <p>The number of minutes to retrieve events for.</p>  <p>Default: 60</p>
    ///   - [`event_categories(Vec<String>)`](crate::client::fluent_builders::DescribeEvents::event_categories) / [`set_event_categories(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeEvents::set_event_categories): <p>A list of event categories that trigger notifications for an event notification subscription.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeEvents::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeEvents::set_filters): <p>This parameter is not currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeEvents::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeEvents::set_max_records): <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeEvents::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeEvents::set_marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeEventsOutput`](crate::output::DescribeEventsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeEventsOutput::marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`events(Option<Vec<Event>>)`](crate::output::DescribeEventsOutput::events): <p>Detailed information about one or more events. </p>
                            /// - On failure, responds with [`SdkError<DescribeEventsError>`](crate::error::DescribeEventsError)
    pub fn describe_events(&self) -> crate::client::fluent_builders::DescribeEvents {
                                crate::client::fluent_builders::DescribeEvents::new(self.handle.clone())
                            }
}

