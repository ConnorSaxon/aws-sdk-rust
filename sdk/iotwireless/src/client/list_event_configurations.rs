// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListEventConfigurations`](crate::client::fluent_builders::ListEventConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_type(EventNotificationResourceType)`](crate::client::fluent_builders::ListEventConfigurations::resource_type) / [`set_resource_type(Option<EventNotificationResourceType>)`](crate::client::fluent_builders::ListEventConfigurations::set_resource_type): <p>Resource type to filter event configurations.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEventConfigurations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListEventConfigurations::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEventConfigurations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEventConfigurations::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
                            /// - On success, responds with [`ListEventConfigurationsOutput`](crate::output::ListEventConfigurationsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListEventConfigurationsOutput::next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`event_configurations_list(Option<Vec<EventConfigurationItem>>)`](crate::output::ListEventConfigurationsOutput::event_configurations_list): <p>Event configurations of all events for a single resource.</p>
                            /// - On failure, responds with [`SdkError<ListEventConfigurationsError>`](crate::error::ListEventConfigurationsError)
    pub fn list_event_configurations(&self) -> crate::client::fluent_builders::ListEventConfigurations {
                                crate::client::fluent_builders::ListEventConfigurations::new(self.handle.clone())
                            }
}

