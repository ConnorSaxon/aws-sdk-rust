// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTimelineEvent`](crate::client::fluent_builders::UpdateTimelineEvent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_client_token): <p>A token ensuring that the operation is called only once with the specified details.</p>
    ///   - [`incident_record_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::incident_record_arn) / [`set_incident_record_arn(Option<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_incident_record_arn): <p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p>
    ///   - [`event_id(impl Into<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::event_id) / [`set_event_id(Option<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_event_id): <p>The ID of the event you are updating. You can find this by using <code>ListTimelineEvents</code>.</p>
    ///   - [`event_time(DateTime)`](crate::client::fluent_builders::UpdateTimelineEvent::event_time) / [`set_event_time(Option<DateTime>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_event_time): <p>The time that the event occurred.</p>
    ///   - [`event_type(impl Into<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::event_type) / [`set_event_type(Option<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_event_type): <p>The type of the event. You can update events of type <code>Custom Event</code>.</p>
    ///   - [`event_data(impl Into<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::event_data) / [`set_event_data(Option<String>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_event_data): <p>A short description of the event.</p>
    ///   - [`event_references(Vec<EventReference>)`](crate::client::fluent_builders::UpdateTimelineEvent::event_references) / [`set_event_references(Option<Vec<EventReference>>)`](crate::client::fluent_builders::UpdateTimelineEvent::set_event_references): <p>Updates all existing references in a <code>TimelineEvent</code>. A reference can be an Amazon Web Services resource involved in the incident or in some way associated with it. When you specify a reference, you enter the Amazon Resource Name (ARN) of the resource. You can also specify a related item. As an example, you could specify the ARN of an Amazon DynamoDB (DynamoDB) table. The table for this example is the resource. You could also specify a Amazon CloudWatch metric for that table. The metric is the related item.</p> <important>   <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes them and enters only new references.</p>  </important>
                            /// - On success, responds with [`UpdateTimelineEventOutput`](crate::output::UpdateTimelineEventOutput)
                            /// - On failure, responds with [`SdkError<UpdateTimelineEventError>`](crate::error::UpdateTimelineEventError)
    pub fn update_timeline_event(&self) -> crate::client::fluent_builders::UpdateTimelineEvent {
                                crate::client::fluent_builders::UpdateTimelineEvent::new(self.handle.clone())
                            }
}

