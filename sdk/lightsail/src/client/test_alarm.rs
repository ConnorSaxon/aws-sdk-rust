// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TestAlarm`](crate::client::fluent_builders::TestAlarm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alarm_name(impl Into<String>)`](crate::client::fluent_builders::TestAlarm::alarm_name) / [`set_alarm_name(Option<String>)`](crate::client::fluent_builders::TestAlarm::set_alarm_name): <p>The name of the alarm to test.</p>
    ///   - [`state(AlarmState)`](crate::client::fluent_builders::TestAlarm::state) / [`set_state(Option<AlarmState>)`](crate::client::fluent_builders::TestAlarm::set_state): <p>The alarm state to test.</p>  <p>An alarm has the following possible states that can be tested:</p>  <ul>   <li> <p> <code>ALARM</code> - The metric is outside of the defined threshold.</p> </li>   <li> <p> <code>INSUFFICIENT_DATA</code> - The alarm has just started, the metric is not available, or not enough data is available for the metric to determine the alarm state.</p> </li>   <li> <p> <code>OK</code> - The metric is within the defined threshold.</p> </li>  </ul>
                            /// - On success, responds with [`TestAlarmOutput`](crate::output::TestAlarmOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::output::TestAlarmOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
                            /// - On failure, responds with [`SdkError<TestAlarmError>`](crate::error::TestAlarmError)
    pub fn test_alarm(&self) -> crate::client::fluent_builders::TestAlarm {
                                crate::client::fluent_builders::TestAlarm::new(self.handle.clone())
                            }
}

