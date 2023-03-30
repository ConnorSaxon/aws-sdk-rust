// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateDeviceDefinition`](crate::client::fluent_builders::UpdateDeviceDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_definition_id(impl Into<String>)`](crate::client::fluent_builders::UpdateDeviceDefinition::device_definition_id) / [`set_device_definition_id(Option<String>)`](crate::client::fluent_builders::UpdateDeviceDefinition::set_device_definition_id): The ID of the device definition.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateDeviceDefinition::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateDeviceDefinition::set_name): The name of the definition.
                            /// - On success, responds with [`UpdateDeviceDefinitionOutput`](crate::output::UpdateDeviceDefinitionOutput)
                            /// - On failure, responds with [`SdkError<UpdateDeviceDefinitionError>`](crate::error::UpdateDeviceDefinitionError)
    pub fn update_device_definition(&self) -> crate::client::fluent_builders::UpdateDeviceDefinition {
                                crate::client::fluent_builders::UpdateDeviceDefinition::new(self.handle.clone())
                            }
}

