// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RefreshSchemas`](crate::client::fluent_builders::RefreshSchemas) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`endpoint_arn(impl Into<String>)`](crate::client::fluent_builders::RefreshSchemas::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::client::fluent_builders::RefreshSchemas::set_endpoint_arn): <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    ///   - [`replication_instance_arn(impl Into<String>)`](crate::client::fluent_builders::RefreshSchemas::replication_instance_arn) / [`set_replication_instance_arn(Option<String>)`](crate::client::fluent_builders::RefreshSchemas::set_replication_instance_arn): <p>The Amazon Resource Name (ARN) of the replication instance.</p>
                            /// - On success, responds with [`RefreshSchemasOutput`](crate::output::RefreshSchemasOutput) with field(s):
    ///   - [`refresh_schemas_status(Option<RefreshSchemasStatus>)`](crate::output::RefreshSchemasOutput::refresh_schemas_status): <p>The status of the refreshed schema.</p>
                            /// - On failure, responds with [`SdkError<RefreshSchemasError>`](crate::error::RefreshSchemasError)
    pub fn refresh_schemas(&self) -> crate::client::fluent_builders::RefreshSchemas {
                                crate::client::fluent_builders::RefreshSchemas::new(self.handle.clone())
                            }
}

