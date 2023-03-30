// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendEvent`](crate::client::fluent_builders::SendEvent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_id(impl Into<String>)`](crate::client::fluent_builders::SendEvent::event_id) / [`set_event_id(Option<String>)`](crate::client::fluent_builders::SendEvent::set_event_id): <p>The event ID to upload.</p>
    ///   - [`event_type_name(impl Into<String>)`](crate::client::fluent_builders::SendEvent::event_type_name) / [`set_event_type_name(Option<String>)`](crate::client::fluent_builders::SendEvent::set_event_type_name): <p>The event type name of the event.</p>
    ///   - [`event_timestamp(impl Into<String>)`](crate::client::fluent_builders::SendEvent::event_timestamp) / [`set_event_timestamp(Option<String>)`](crate::client::fluent_builders::SendEvent::set_event_timestamp): <p>The timestamp that defines when the event under evaluation occurred. The timestamp must be specified using ISO 8601 standard in UTC.</p>
    ///   - [`event_variables(HashMap<String, String>)`](crate::client::fluent_builders::SendEvent::event_variables) / [`set_event_variables(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SendEvent::set_event_variables): <p>Names of the event type's variables you defined in Amazon Fraud Detector to represent data elements and their corresponding values for the event you are sending for evaluation.</p>
    ///   - [`assigned_label(impl Into<String>)`](crate::client::fluent_builders::SendEvent::assigned_label) / [`set_assigned_label(Option<String>)`](crate::client::fluent_builders::SendEvent::set_assigned_label): <p>The label to associate with the event. Required if specifying <code>labelTimestamp</code>.</p>
    ///   - [`label_timestamp(impl Into<String>)`](crate::client::fluent_builders::SendEvent::label_timestamp) / [`set_label_timestamp(Option<String>)`](crate::client::fluent_builders::SendEvent::set_label_timestamp): <p>The timestamp associated with the label. Required if specifying <code>assignedLabel</code>.</p>
    ///   - [`entities(Vec<Entity>)`](crate::client::fluent_builders::SendEvent::entities) / [`set_entities(Option<Vec<Entity>>)`](crate::client::fluent_builders::SendEvent::set_entities): <p>An array of entities.</p>
                            /// - On success, responds with [`SendEventOutput`](crate::output::SendEventOutput)
                            /// - On failure, responds with [`SdkError<SendEventError>`](crate::error::SendEventError)
    pub fn send_event(&self) -> crate::client::fluent_builders::SendEvent {
                                crate::client::fluent_builders::SendEvent::new(self.handle.clone())
                            }
}

