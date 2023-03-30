// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDeviceDefinition`](crate::client::fluent_builders::CreateDeviceDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl Into<String>)`](crate::client::fluent_builders::CreateDeviceDefinition::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::client::fluent_builders::CreateDeviceDefinition::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`initial_version(DeviceDefinitionVersion)`](crate::client::fluent_builders::CreateDeviceDefinition::initial_version) / [`set_initial_version(Option<DeviceDefinitionVersion>)`](crate::client::fluent_builders::CreateDeviceDefinition::set_initial_version): Information about the initial version of the device definition.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateDeviceDefinition::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateDeviceDefinition::set_name): The name of the device definition.
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateDeviceDefinition::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateDeviceDefinition::set_tags): Tag(s) to add to the new resource.
                            /// - On success, responds with [`CreateDeviceDefinitionOutput`](crate::output::CreateDeviceDefinitionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::arn): The ARN of the definition.
    ///   - [`creation_timestamp(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the definition was created.
    ///   - [`id(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::id): The ID of the definition.
    ///   - [`last_updated_timestamp(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::last_updated_timestamp): The time, in milliseconds since the epoch, when the definition was last updated.
    ///   - [`latest_version(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::latest_version): The ID of the latest version associated with the definition.
    ///   - [`latest_version_arn(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::latest_version_arn): The ARN of the latest version associated with the definition.
    ///   - [`name(Option<String>)`](crate::output::CreateDeviceDefinitionOutput::name): The name of the definition.
                            /// - On failure, responds with [`SdkError<CreateDeviceDefinitionError>`](crate::error::CreateDeviceDefinitionError)
    pub fn create_device_definition(&self) -> crate::client::fluent_builders::CreateDeviceDefinition {
                                crate::client::fluent_builders::CreateDeviceDefinition::new(self.handle.clone())
                            }
}

