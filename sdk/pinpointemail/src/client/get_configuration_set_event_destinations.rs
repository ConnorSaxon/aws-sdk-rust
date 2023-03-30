// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetConfigurationSetEventDestinations`](crate::client::fluent_builders::GetConfigurationSetEventDestinations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::GetConfigurationSetEventDestinations::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::GetConfigurationSetEventDestinations::set_configuration_set_name): <p>The name of the configuration set that contains the event destination.</p>
                            /// - On success, responds with [`GetConfigurationSetEventDestinationsOutput`](crate::output::GetConfigurationSetEventDestinationsOutput) with field(s):
    ///   - [`event_destinations(Option<Vec<EventDestination>>)`](crate::output::GetConfigurationSetEventDestinationsOutput::event_destinations): <p>An array that includes all of the events destinations that have been configured for the configuration set.</p>
                            /// - On failure, responds with [`SdkError<GetConfigurationSetEventDestinationsError>`](crate::error::GetConfigurationSetEventDestinationsError)
    pub fn get_configuration_set_event_destinations(&self) -> crate::client::fluent_builders::GetConfigurationSetEventDestinations {
                                crate::client::fluent_builders::GetConfigurationSetEventDestinations::new(self.handle.clone())
                            }
}

