// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSchemaAsJson`](crate::client::fluent_builders::GetSchemaAsJson) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schema_arn(impl Into<String>)`](crate::client::fluent_builders::GetSchemaAsJson::schema_arn) / [`set_schema_arn(Option<String>)`](crate::client::fluent_builders::GetSchemaAsJson::set_schema_arn): <p>The ARN of the schema to retrieve.</p>
                            /// - On success, responds with [`GetSchemaAsJsonOutput`](crate::output::GetSchemaAsJsonOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::GetSchemaAsJsonOutput::name): <p>The name of the retrieved schema.</p>
    ///   - [`document(Option<String>)`](crate::output::GetSchemaAsJsonOutput::document): <p>The JSON representation of the schema document.</p>
                            /// - On failure, responds with [`SdkError<GetSchemaAsJsonError>`](crate::error::GetSchemaAsJsonError)
    pub fn get_schema_as_json(&self) -> crate::client::fluent_builders::GetSchemaAsJson {
                                crate::client::fluent_builders::GetSchemaAsJson::new(self.handle.clone())
                            }
}

