// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDisableAlarm`](crate::client::fluent_builders::BatchDisableAlarm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`disable_action_requests(Vec<DisableAlarmActionRequest>)`](crate::client::fluent_builders::BatchDisableAlarm::disable_action_requests) / [`set_disable_action_requests(Option<Vec<DisableAlarmActionRequest>>)`](crate::client::fluent_builders::BatchDisableAlarm::set_disable_action_requests): <p>The list of disable action requests. You can specify up to 10 requests per operation.</p>
                            /// - On success, responds with [`BatchDisableAlarmOutput`](crate::output::BatchDisableAlarmOutput) with field(s):
    ///   - [`error_entries(Option<Vec<BatchAlarmActionErrorEntry>>)`](crate::output::BatchDisableAlarmOutput::error_entries): <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
                            /// - On failure, responds with [`SdkError<BatchDisableAlarmError>`](crate::error::BatchDisableAlarmError)
    pub fn batch_disable_alarm(&self) -> crate::client::fluent_builders::BatchDisableAlarm {
                                crate::client::fluent_builders::BatchDisableAlarm::new(self.handle.clone())
                            }
}

