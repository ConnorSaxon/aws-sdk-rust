// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateIndexingConfiguration`](crate::client::fluent_builders::UpdateIndexingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`thing_indexing_configuration(ThingIndexingConfiguration)`](crate::client::fluent_builders::UpdateIndexingConfiguration::thing_indexing_configuration) / [`set_thing_indexing_configuration(Option<ThingIndexingConfiguration>)`](crate::client::fluent_builders::UpdateIndexingConfiguration::set_thing_indexing_configuration): <p>Thing indexing configuration.</p>
    ///   - [`thing_group_indexing_configuration(ThingGroupIndexingConfiguration)`](crate::client::fluent_builders::UpdateIndexingConfiguration::thing_group_indexing_configuration) / [`set_thing_group_indexing_configuration(Option<ThingGroupIndexingConfiguration>)`](crate::client::fluent_builders::UpdateIndexingConfiguration::set_thing_group_indexing_configuration): <p>Thing group indexing configuration.</p>
                            /// - On success, responds with [`UpdateIndexingConfigurationOutput`](crate::output::UpdateIndexingConfigurationOutput)
                            /// - On failure, responds with [`SdkError<UpdateIndexingConfigurationError>`](crate::error::UpdateIndexingConfigurationError)
    pub fn update_indexing_configuration(&self) -> crate::client::fluent_builders::UpdateIndexingConfiguration {
                                crate::client::fluent_builders::UpdateIndexingConfiguration::new(self.handle.clone())
                            }
}

