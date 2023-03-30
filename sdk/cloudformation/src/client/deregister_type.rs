// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterType`](crate::client::fluent_builders::DeregisterType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DeregisterType::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DeregisterType::set_arn): <p>The Amazon Resource Name (ARN) of the extension.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`r#type(RegistryType)`](crate::client::fluent_builders::DeregisterType::type) / [`set_type(Option<RegistryType>)`](crate::client::fluent_builders::DeregisterType::set_type): <p>The kind of extension.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`type_name(impl Into<String>)`](crate::client::fluent_builders::DeregisterType::type_name) / [`set_type_name(Option<String>)`](crate::client::fluent_builders::DeregisterType::set_type_name): <p>The name of the extension.</p>  <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    ///   - [`version_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterType::version_id) / [`set_version_id(Option<String>)`](crate::client::fluent_builders::DeregisterType::set_version_id): <p>The ID of a specific version of the extension. The version ID is the value at the end of the Amazon Resource Name (ARN) assigned to the extension version when it is registered.</p>
                            /// - On success, responds with [`DeregisterTypeOutput`](crate::output::DeregisterTypeOutput)
                            /// - On failure, responds with [`SdkError<DeregisterTypeError>`](crate::error::DeregisterTypeError)
    pub fn deregister_type(&self) -> crate::client::fluent_builders::DeregisterType {
                                crate::client::fluent_builders::DeregisterType::new(self.handle.clone())
                            }
}

