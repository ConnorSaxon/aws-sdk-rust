// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeMaintenanceWindowSchedule`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`window_id(impl Into<String>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::window_id) / [`set_window_id(Option<String>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_window_id): <p>The ID of the maintenance window to retrieve information about.</p>
    ///   - [`targets(Vec<Target>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::targets) / [`set_targets(Option<Vec<Target>>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_targets): <p>The managed node ID or key-value pair to retrieve information about.</p>
    ///   - [`resource_type(MaintenanceWindowResourceType)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::resource_type) / [`set_resource_type(Option<MaintenanceWindowResourceType>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_resource_type): <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    ///   - [`filters(Vec<PatchOrchestratorFilter>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::filters) / [`set_filters(Option<Vec<PatchOrchestratorFilter>>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_filters): <p>Filters used to limit the range of results. For example, you can limit maintenance window executions to only those scheduled before or after a certain date and time.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::set_next_token): <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
                            /// - On success, responds with [`DescribeMaintenanceWindowScheduleOutput`](crate::output::DescribeMaintenanceWindowScheduleOutput) with field(s):
    ///   - [`scheduled_window_executions(Option<Vec<ScheduledWindowExecution>>)`](crate::output::DescribeMaintenanceWindowScheduleOutput::scheduled_window_executions): <p>Information about maintenance window executions scheduled for the specified time range.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeMaintenanceWindowScheduleOutput::next_token): <p>The token for the next set of items to return. (You use this token in the next call.)</p>
                            /// - On failure, responds with [`SdkError<DescribeMaintenanceWindowScheduleError>`](crate::error::DescribeMaintenanceWindowScheduleError)
    pub fn describe_maintenance_window_schedule(&self) -> crate::client::fluent_builders::DescribeMaintenanceWindowSchedule {
                                crate::client::fluent_builders::DescribeMaintenanceWindowSchedule::new(self.handle.clone())
                            }
}

