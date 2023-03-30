// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConfigurationSetEventDestination`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::set_configuration_set_name): ConfigurationSetName
    ///   - [`event_destination(EventDestinationDefinition)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::event_destination) / [`set_event_destination(Option<EventDestinationDefinition>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::set_event_destination): An object that defines a single event destination.
    ///   - [`event_destination_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::event_destination_name) / [`set_event_destination_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination::set_event_destination_name): EventDestinationName
                            /// - On success, responds with [`UpdateConfigurationSetEventDestinationOutput`](crate::output::UpdateConfigurationSetEventDestinationOutput)
                            /// - On failure, responds with [`SdkError<UpdateConfigurationSetEventDestinationError>`](crate::error::UpdateConfigurationSetEventDestinationError)
    pub fn update_configuration_set_event_destination(&self) -> crate::client::fluent_builders::UpdateConfigurationSetEventDestination {
                                crate::client::fluent_builders::UpdateConfigurationSetEventDestination::new(self.handle.clone())
                            }
}

