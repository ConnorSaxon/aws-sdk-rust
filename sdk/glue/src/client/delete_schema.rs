// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSchema`](crate::client::fluent_builders::DeleteSchema) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schema_id(SchemaId)`](crate::client::fluent_builders::DeleteSchema::schema_id) / [`set_schema_id(Option<SchemaId>)`](crate::client::fluent_builders::DeleteSchema::set_schema_id): <p>This is a wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
                            /// - On success, responds with [`DeleteSchemaOutput`](crate::output::DeleteSchemaOutput) with field(s):
    ///   - [`schema_arn(Option<String>)`](crate::output::DeleteSchemaOutput::schema_arn): <p>The Amazon Resource Name (ARN) of the schema being deleted.</p>
    ///   - [`schema_name(Option<String>)`](crate::output::DeleteSchemaOutput::schema_name): <p>The name of the schema being deleted.</p>
    ///   - [`status(Option<SchemaStatus>)`](crate::output::DeleteSchemaOutput::status): <p>The status of the schema.</p>
                            /// - On failure, responds with [`SdkError<DeleteSchemaError>`](crate::error::DeleteSchemaError)
    pub fn delete_schema(&self) -> crate::client::fluent_builders::DeleteSchema {
                                crate::client::fluent_builders::DeleteSchema::new(self.handle.clone())
                            }
}

