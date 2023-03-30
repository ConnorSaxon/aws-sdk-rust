// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateEventConfigurations`](crate::client::fluent_builders::UpdateEventConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_configurations(HashMap<EventType, Configuration>)`](crate::client::fluent_builders::UpdateEventConfigurations::event_configurations) / [`set_event_configurations(Option<HashMap<EventType, Configuration>>)`](crate::client::fluent_builders::UpdateEventConfigurations::set_event_configurations): <p>The new event configuration values.</p>
                            /// - On success, responds with [`UpdateEventConfigurationsOutput`](crate::output::UpdateEventConfigurationsOutput)
                            /// - On failure, responds with [`SdkError<UpdateEventConfigurationsError>`](crate::error::UpdateEventConfigurationsError)
    pub fn update_event_configurations(&self) -> crate::client::fluent_builders::UpdateEventConfigurations {
                                crate::client::fluent_builders::UpdateEventConfigurations::new(self.handle.clone())
                            }
}

