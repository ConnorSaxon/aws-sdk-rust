// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCoreDefinitionVersion`](crate::client::fluent_builders::CreateCoreDefinitionVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl Into<String>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`core_definition_id(impl Into<String>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::core_definition_id) / [`set_core_definition_id(Option<String>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::set_core_definition_id): The ID of the core definition.
    ///   - [`cores(Vec<Core>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::cores) / [`set_cores(Option<Vec<Core>>)`](crate::client::fluent_builders::CreateCoreDefinitionVersion::set_cores): A list of cores in the core definition version.
                            /// - On success, responds with [`CreateCoreDefinitionVersionOutput`](crate::output::CreateCoreDefinitionVersionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateCoreDefinitionVersionOutput::arn): The ARN of the version.
    ///   - [`creation_timestamp(Option<String>)`](crate::output::CreateCoreDefinitionVersionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the version was created.
    ///   - [`id(Option<String>)`](crate::output::CreateCoreDefinitionVersionOutput::id): The ID of the parent definition that the version is associated with.
    ///   - [`version(Option<String>)`](crate::output::CreateCoreDefinitionVersionOutput::version): The ID of the version.
                            /// - On failure, responds with [`SdkError<CreateCoreDefinitionVersionError>`](crate::error::CreateCoreDefinitionVersionError)
    pub fn create_core_definition_version(&self) -> crate::client::fluent_builders::CreateCoreDefinitionVersion {
                                crate::client::fluent_builders::CreateCoreDefinitionVersion::new(self.handle.clone())
                            }
}

