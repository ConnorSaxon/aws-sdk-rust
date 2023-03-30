// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeScheduledActions`](crate::client::fluent_builders::DescribeScheduledActions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeScheduledActions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`scheduled_action_name(impl Into<String>)`](crate::client::fluent_builders::DescribeScheduledActions::scheduled_action_name) / [`set_scheduled_action_name(Option<String>)`](crate::client::fluent_builders::DescribeScheduledActions::set_scheduled_action_name): <p>The name of the scheduled action to retrieve. </p>
    ///   - [`target_action_type(ScheduledActionTypeValues)`](crate::client::fluent_builders::DescribeScheduledActions::target_action_type) / [`set_target_action_type(Option<ScheduledActionTypeValues>)`](crate::client::fluent_builders::DescribeScheduledActions::set_target_action_type): <p>The type of the scheduled actions to retrieve. </p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::DescribeScheduledActions::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeScheduledActions::set_start_time): <p>The start time in UTC of the scheduled actions to retrieve. Only active scheduled actions that have invocations after this time are retrieved.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::DescribeScheduledActions::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::DescribeScheduledActions::set_end_time): <p>The end time in UTC of the scheduled action to retrieve. Only active scheduled actions that have invocations before this time are retrieved.</p>
    ///   - [`active(bool)`](crate::client::fluent_builders::DescribeScheduledActions::active) / [`set_active(Option<bool>)`](crate::client::fluent_builders::DescribeScheduledActions::set_active): <p>If true, retrieve only active scheduled actions. If false, retrieve only disabled scheduled actions. </p>
    ///   - [`filters(Vec<ScheduledActionFilter>)`](crate::client::fluent_builders::DescribeScheduledActions::filters) / [`set_filters(Option<Vec<ScheduledActionFilter>>)`](crate::client::fluent_builders::DescribeScheduledActions::set_filters): <p>List of scheduled action filters. </p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeScheduledActions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeScheduledActions::set_marker): <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeScheduledActions</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeScheduledActions::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeScheduledActions::set_max_records): <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p>  <p>Default: <code>100</code> </p>  <p>Constraints: minimum 20, maximum 100.</p>
                            /// - On success, responds with [`DescribeScheduledActionsOutput`](crate::output::DescribeScheduledActionsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeScheduledActionsOutput::marker): <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeScheduledActions</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    ///   - [`scheduled_actions(Option<Vec<ScheduledAction>>)`](crate::output::DescribeScheduledActionsOutput::scheduled_actions): <p>List of retrieved scheduled actions. </p>
                            /// - On failure, responds with [`SdkError<DescribeScheduledActionsError>`](crate::error::DescribeScheduledActionsError)
    pub fn describe_scheduled_actions(&self) -> crate::client::fluent_builders::DescribeScheduledActions {
                                crate::client::fluent_builders::DescribeScheduledActions::new(self.handle.clone())
                            }
}

