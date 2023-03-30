// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPositionConfigurations`](crate::client::fluent_builders::ListPositionConfigurations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPositionConfigurations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_type(PositionResourceType)`](crate::client::fluent_builders::ListPositionConfigurations::resource_type) / [`set_resource_type(Option<PositionResourceType>)`](crate::client::fluent_builders::ListPositionConfigurations::set_resource_type): <p>Resource type for which position configurations are listed.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPositionConfigurations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListPositionConfigurations::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPositionConfigurations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPositionConfigurations::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
                            /// - On success, responds with [`ListPositionConfigurationsOutput`](crate::output::ListPositionConfigurationsOutput) with field(s):
    ///   - [`position_configuration_list(Option<Vec<PositionConfigurationItem>>)`](crate::output::ListPositionConfigurationsOutput::position_configuration_list): <p>A list of position configurations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPositionConfigurationsOutput::next_token): <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListPositionConfigurationsError>`](crate::error::ListPositionConfigurationsError)
    #[deprecated(note = "This operation is no longer supported.")]
    pub fn list_position_configurations(&self) -> crate::client::fluent_builders::ListPositionConfigurations {
                                crate::client::fluent_builders::ListPositionConfigurations::new(self.handle.clone())
                            }
}

