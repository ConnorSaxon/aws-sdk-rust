// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetTemporaryGlueTableCredentials`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`table_arn(impl Into<String>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::table_arn) / [`set_table_arn(Option<String>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::set_table_arn): <p>The ARN identifying a table in the Data Catalog for the temporary credentials request.</p>
    ///   - [`permissions(Vec<Permission>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::permissions) / [`set_permissions(Option<Vec<Permission>>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::set_permissions): <p>Filters the request based on the user having been granted a list of specified permissions on the requested resource(s).</p>
    ///   - [`duration_seconds(i32)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::set_duration_seconds): <p>The time period, between 900 and 21,600 seconds, for the timeout of the temporary credentials.</p>
    ///   - [`audit_context(AuditContext)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::audit_context) / [`set_audit_context(Option<AuditContext>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::set_audit_context): <p>A structure representing context to access a resource (column names, query ID, etc).</p>
    ///   - [`supported_permission_types(Vec<PermissionType>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::supported_permission_types) / [`set_supported_permission_types(Option<Vec<PermissionType>>)`](crate::client::fluent_builders::GetTemporaryGlueTableCredentials::set_supported_permission_types): <p>A list of supported permission types for the table. Valid values are <code>COLUMN_PERMISSION</code> and <code>CELL_FILTER_PERMISSION</code>.</p>
                            /// - On success, responds with [`GetTemporaryGlueTableCredentialsOutput`](crate::output::GetTemporaryGlueTableCredentialsOutput) with field(s):
    ///   - [`access_key_id(Option<String>)`](crate::output::GetTemporaryGlueTableCredentialsOutput::access_key_id): <p>The access key ID for the temporary credentials.</p>
    ///   - [`secret_access_key(Option<String>)`](crate::output::GetTemporaryGlueTableCredentialsOutput::secret_access_key): <p>The secret key for the temporary credentials.</p>
    ///   - [`session_token(Option<String>)`](crate::output::GetTemporaryGlueTableCredentialsOutput::session_token): <p>The session token for the temporary credentials.</p>
    ///   - [`expiration(Option<DateTime>)`](crate::output::GetTemporaryGlueTableCredentialsOutput::expiration): <p>The date and time when the temporary credentials expire.</p>
                            /// - On failure, responds with [`SdkError<GetTemporaryGlueTableCredentialsError>`](crate::error::GetTemporaryGlueTableCredentialsError)
    pub fn get_temporary_glue_table_credentials(&self) -> crate::client::fluent_builders::GetTemporaryGlueTableCredentials {
                                crate::client::fluent_builders::GetTemporaryGlueTableCredentials::new(self.handle.clone())
                            }
}

