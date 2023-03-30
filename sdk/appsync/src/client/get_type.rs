// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetType`](crate::client::fluent_builders::GetType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::GetType::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::GetType::set_api_id): <p>The API ID.</p>
    ///   - [`type_name(impl Into<String>)`](crate::client::fluent_builders::GetType::type_name) / [`set_type_name(Option<String>)`](crate::client::fluent_builders::GetType::set_type_name): <p>The type name.</p>
    ///   - [`format(TypeDefinitionFormat)`](crate::client::fluent_builders::GetType::format) / [`set_format(Option<TypeDefinitionFormat>)`](crate::client::fluent_builders::GetType::set_format): <p>The type format: SDL or JSON.</p>
                            /// - On success, responds with [`GetTypeOutput`](crate::output::GetTypeOutput) with field(s):
    ///   - [`r#type(Option<Type>)`](crate::output::GetTypeOutput::type): <p>The <code>Type</code> object.</p>
                            /// - On failure, responds with [`SdkError<GetTypeError>`](crate::error::GetTypeError)
    pub fn get_type(&self) -> crate::client::fluent_builders::GetType {
                                crate::client::fluent_builders::GetType::new(self.handle.clone())
                            }
}

