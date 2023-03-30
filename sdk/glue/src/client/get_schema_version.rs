// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSchemaVersion`](crate::client::fluent_builders::GetSchemaVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`schema_id(SchemaId)`](crate::client::fluent_builders::GetSchemaVersion::schema_id) / [`set_schema_id(Option<SchemaId>)`](crate::client::fluent_builders::GetSchemaVersion::set_schema_id): <p>This is a wrapper structure to contain schema identity fields. The structure contains:</p>  <ul>   <li> <p>SchemaId$SchemaArn: The Amazon Resource Name (ARN) of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>   <li> <p>SchemaId$SchemaName: The name of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>  </ul>
    ///   - [`schema_version_id(impl Into<String>)`](crate::client::fluent_builders::GetSchemaVersion::schema_version_id) / [`set_schema_version_id(Option<String>)`](crate::client::fluent_builders::GetSchemaVersion::set_schema_version_id): <p>The <code>SchemaVersionId</code> of the schema version. This field is required for fetching by schema ID. Either this or the <code>SchemaId</code> wrapper has to be provided.</p>
    ///   - [`schema_version_number(SchemaVersionNumber)`](crate::client::fluent_builders::GetSchemaVersion::schema_version_number) / [`set_schema_version_number(Option<SchemaVersionNumber>)`](crate::client::fluent_builders::GetSchemaVersion::set_schema_version_number): <p>The version number of the schema.</p>
                            /// - On success, responds with [`GetSchemaVersionOutput`](crate::output::GetSchemaVersionOutput) with field(s):
    ///   - [`schema_version_id(Option<String>)`](crate::output::GetSchemaVersionOutput::schema_version_id): <p>The <code>SchemaVersionId</code> of the schema version.</p>
    ///   - [`schema_definition(Option<String>)`](crate::output::GetSchemaVersionOutput::schema_definition): <p>The schema definition for the schema ID.</p>
    ///   - [`data_format(Option<DataFormat>)`](crate::output::GetSchemaVersionOutput::data_format): <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    ///   - [`schema_arn(Option<String>)`](crate::output::GetSchemaVersionOutput::schema_arn): <p>The Amazon Resource Name (ARN) of the schema.</p>
    ///   - [`version_number(i64)`](crate::output::GetSchemaVersionOutput::version_number): <p>The version number of the schema.</p>
    ///   - [`status(Option<SchemaVersionStatus>)`](crate::output::GetSchemaVersionOutput::status): <p>The status of the schema version. </p>
    ///   - [`created_time(Option<String>)`](crate::output::GetSchemaVersionOutput::created_time): <p>The date and time the schema version was created.</p>
                            /// - On failure, responds with [`SdkError<GetSchemaVersionError>`](crate::error::GetSchemaVersionError)
    pub fn get_schema_version(&self) -> crate::client::fluent_builders::GetSchemaVersion {
                                crate::client::fluent_builders::GetSchemaVersion::new(self.handle.clone())
                            }
}

