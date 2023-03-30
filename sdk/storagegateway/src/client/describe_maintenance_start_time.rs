// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeMaintenanceStartTime`](crate::client::fluent_builders::DescribeMaintenanceStartTime) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeMaintenanceStartTime::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::DescribeMaintenanceStartTime::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On success, responds with [`DescribeMaintenanceStartTimeOutput`](crate::output::DescribeMaintenanceStartTimeOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::DescribeMaintenanceStartTimeOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`hour_of_day(Option<i32>)`](crate::output::DescribeMaintenanceStartTimeOutput::hour_of_day): <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    ///   - [`minute_of_hour(Option<i32>)`](crate::output::DescribeMaintenanceStartTimeOutput::minute_of_hour): <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (0 to 59). The minute of the hour is in the time zone of the gateway.</p>
    ///   - [`day_of_week(Option<i32>)`](crate::output::DescribeMaintenanceStartTimeOutput::day_of_week): <p>An ordinal number between 0 and 6 that represents the day of the week, where 0 represents Sunday and 6 represents Saturday. The day of week is in the time zone of the gateway.</p>
    ///   - [`day_of_month(Option<i32>)`](crate::output::DescribeMaintenanceStartTimeOutput::day_of_month): <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    ///   - [`timezone(Option<String>)`](crate::output::DescribeMaintenanceStartTimeOutput::timezone): <p>A value that indicates the time zone that is set for the gateway. The start time and day of week specified should be in the time zone of the gateway.</p>
                            /// - On failure, responds with [`SdkError<DescribeMaintenanceStartTimeError>`](crate::error::DescribeMaintenanceStartTimeError)
    pub fn describe_maintenance_start_time(&self) -> crate::client::fluent_builders::DescribeMaintenanceStartTime {
                                crate::client::fluent_builders::DescribeMaintenanceStartTime::new(self.handle.clone())
                            }
}

