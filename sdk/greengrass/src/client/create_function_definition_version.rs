// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFunctionDefinitionVersion`](crate::client::fluent_builders::CreateFunctionDefinitionVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl Into<String>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`default_config(FunctionDefaultConfig)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::default_config) / [`set_default_config(Option<FunctionDefaultConfig>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::set_default_config): The default configuration that applies to all Lambda functions in this function definition version. Individual Lambda functions can override these settings.
    ///   - [`function_definition_id(impl Into<String>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::function_definition_id) / [`set_function_definition_id(Option<String>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::set_function_definition_id): The ID of the Lambda function definition.
    ///   - [`functions(Vec<Function>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::functions) / [`set_functions(Option<Vec<Function>>)`](crate::client::fluent_builders::CreateFunctionDefinitionVersion::set_functions): A list of Lambda functions in this function definition version.
                            /// - On success, responds with [`CreateFunctionDefinitionVersionOutput`](crate::output::CreateFunctionDefinitionVersionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateFunctionDefinitionVersionOutput::arn): The ARN of the version.
    ///   - [`creation_timestamp(Option<String>)`](crate::output::CreateFunctionDefinitionVersionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the version was created.
    ///   - [`id(Option<String>)`](crate::output::CreateFunctionDefinitionVersionOutput::id): The ID of the parent definition that the version is associated with.
    ///   - [`version(Option<String>)`](crate::output::CreateFunctionDefinitionVersionOutput::version): The ID of the version.
                            /// - On failure, responds with [`SdkError<CreateFunctionDefinitionVersionError>`](crate::error::CreateFunctionDefinitionVersionError)
    pub fn create_function_definition_version(&self) -> crate::client::fluent_builders::CreateFunctionDefinitionVersion {
                                crate::client::fluent_builders::CreateFunctionDefinitionVersion::new(self.handle.clone())
                            }
}

