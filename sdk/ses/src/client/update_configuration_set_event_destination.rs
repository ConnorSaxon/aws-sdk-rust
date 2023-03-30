// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConfigurationSetEventDestination`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::set_configuration_set_name): <p>The name of the configuration set that contains the event destination that you want to update.</p>
    ///   - [`event_destination(EventDestination)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::event_destination) / [`set_event_destination(Option<EventDestination>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::set_event_destination): <p>The event destination object that you want to apply to the specified configuration set.</p>
                            /// - On success, responds with [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput)
                            /// - On failure, responds with [`SdkError<UpdateConfigurationSetEventDestinationError>`](crate::error::UpdateConfigurationSetEventDestinationError)
    pub fn update_configuration_set_event_destination(&self) -> crate::client::fluent_builders::UpdateConfigurationSetEventDestination {
                                crate::client::fluent_builders::UpdateConfigurationSetEventDestination::new(self.handle.clone())
                            }
}

