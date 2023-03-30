// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateMaintenanceStartTime`](crate::client::fluent_builders::UpdateMaintenanceStartTime) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`hour_of_day(i32)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::hour_of_day) / [`set_hour_of_day(Option<i32>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::set_hour_of_day): <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    ///   - [`minute_of_hour(i32)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::minute_of_hour) / [`set_minute_of_hour(Option<i32>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::set_minute_of_hour): <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    ///   - [`day_of_week(i32)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::day_of_week) / [`set_day_of_week(Option<i32>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::set_day_of_week): <p>The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    ///   - [`day_of_month(i32)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::day_of_month) / [`set_day_of_month(Option<i32>)`](crate::client::fluent_builders::UpdateMaintenanceStartTime::set_day_of_month): <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
                            /// - On success, responds with [`UpdateMaintenanceStartTimeOutput`](crate::output::UpdateMaintenanceStartTimeOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::UpdateMaintenanceStartTimeOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<UpdateMaintenanceStartTimeError>`](crate::error::UpdateMaintenanceStartTimeError)
    pub fn update_maintenance_start_time(&self) -> crate::client::fluent_builders::UpdateMaintenanceStartTime {
                                crate::client::fluent_builders::UpdateMaintenanceStartTime::new(self.handle.clone())
                            }
}

