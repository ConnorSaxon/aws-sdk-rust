// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateMaintenanceWindow`](crate::client::fluent_builders::CreateMaintenanceWindow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_name): <p>The name of the maintenance window.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_description): <p>An optional description for the maintenance window. We recommend specifying a description to help you organize your maintenance windows. </p>
    ///   - [`start_date(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::start_date) / [`set_start_date(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_start_date): <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to become active. <code>StartDate</code> allows you to delay activation of the maintenance window until the specified future date.</p>
    ///   - [`end_date(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::end_date) / [`set_end_date(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_end_date): <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to become inactive. <code>EndDate</code> allows you to set a date and time in the future when the maintenance window will no longer run.</p>
    ///   - [`schedule(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::schedule) / [`set_schedule(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_schedule): <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    ///   - [`schedule_timezone(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::schedule_timezone) / [`set_schedule_timezone(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_schedule_timezone): <p>The time zone that the scheduled maintenance window executions are based on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p>
    ///   - [`schedule_offset(i32)`](crate::client::fluent_builders::CreateMaintenanceWindow::schedule_offset) / [`set_schedule_offset(Option<i32>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_schedule_offset): <p>The number of days to wait after the date and time specified by a cron expression before running the maintenance window.</p>  <p>For example, the following cron expression schedules a maintenance window to run on the third Tuesday of every month at 11:30 PM.</p>  <p> <code>cron(30 23 ? * TUE#3 *)</code> </p>  <p>If the schedule offset is <code>2</code>, the maintenance window won't run until two days later.</p>
    ///   - [`duration(i32)`](crate::client::fluent_builders::CreateMaintenanceWindow::duration) / [`set_duration(i32)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_duration): <p>The duration of the maintenance window in hours.</p>
    ///   - [`cutoff(i32)`](crate::client::fluent_builders::CreateMaintenanceWindow::cutoff) / [`set_cutoff(i32)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_cutoff): <p>The number of hours before the end of the maintenance window that Amazon Web Services Systems Manager stops scheduling new tasks for execution.</p>
    ///   - [`allow_unassociated_targets(bool)`](crate::client::fluent_builders::CreateMaintenanceWindow::allow_unassociated_targets) / [`set_allow_unassociated_targets(bool)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_allow_unassociated_targets): <p>Enables a maintenance window task to run on managed nodes, even if you haven't registered those nodes as targets. If enabled, then you must specify the unregistered managed nodes (by node ID) when you register a task with the maintenance window.</p>  <p>If you don't enable this option, then you must specify previously-registered targets when you register a task with the maintenance window.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_client_token): <p>User-provided idempotency token.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateMaintenanceWindow::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateMaintenanceWindow::set_tags): <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag a maintenance window to identify the type of tasks it will run, the types of targets, and the environment it will run in. In this case, you could specify the following key-value pairs:</p>  <ul>   <li> <p> <code>Key=TaskType,Value=AgentUpdate</code> </p> </li>   <li> <p> <code>Key=OS,Value=Windows</code> </p> </li>   <li> <p> <code>Key=Environment,Value=Production</code> </p> </li>  </ul> <note>   <p>To add tags to an existing maintenance window, use the <code>AddTagsToResource</code> operation.</p>  </note>
                            /// - On success, responds with [`CreateMaintenanceWindowOutput`](crate::output::CreateMaintenanceWindowOutput) with field(s):
    ///   - [`window_id(Option<String>)`](crate::output::CreateMaintenanceWindowOutput::window_id): <p>The ID of the created maintenance window.</p>
                            /// - On failure, responds with [`SdkError<CreateMaintenanceWindowError>`](crate::error::CreateMaintenanceWindowError)
    pub fn create_maintenance_window(&self) -> crate::client::fluent_builders::CreateMaintenanceWindow {
                                crate::client::fluent_builders::CreateMaintenanceWindow::new(self.handle.clone())
                            }
}

