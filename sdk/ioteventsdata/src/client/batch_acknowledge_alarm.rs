// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchAcknowledgeAlarm`](crate::client::fluent_builders::BatchAcknowledgeAlarm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`acknowledge_action_requests(Vec<AcknowledgeAlarmActionRequest>)`](crate::client::fluent_builders::BatchAcknowledgeAlarm::acknowledge_action_requests) / [`set_acknowledge_action_requests(Option<Vec<AcknowledgeAlarmActionRequest>>)`](crate::client::fluent_builders::BatchAcknowledgeAlarm::set_acknowledge_action_requests): <p>The list of acknowledge action requests. You can specify up to 10 requests per operation.</p>
                            /// - On success, responds with [`BatchAcknowledgeAlarmOutput`](crate::output::BatchAcknowledgeAlarmOutput) with field(s):
    ///   - [`error_entries(Option<Vec<BatchAlarmActionErrorEntry>>)`](crate::output::BatchAcknowledgeAlarmOutput::error_entries): <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
                            /// - On failure, responds with [`SdkError<BatchAcknowledgeAlarmError>`](crate::error::BatchAcknowledgeAlarmError)
    pub fn batch_acknowledge_alarm(&self) -> crate::client::fluent_builders::BatchAcknowledgeAlarm {
                                crate::client::fluent_builders::BatchAcknowledgeAlarm::new(self.handle.clone())
                            }
}

