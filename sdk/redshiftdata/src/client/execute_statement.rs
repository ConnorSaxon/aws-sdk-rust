// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ExecuteStatement`](crate::client::fluent_builders::ExecuteStatement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`sql(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::sql) / [`set_sql(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_sql): <p>The SQL statement text to run. </p>
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_cluster_identifier): <p>The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials. </p>
    ///   - [`secret_arn(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::secret_arn) / [`set_secret_arn(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_secret_arn): <p>The name or ARN of the secret that enables access to the database. This parameter is required when authenticating using Secrets Manager. </p>
    ///   - [`db_user(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::db_user) / [`set_db_user(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_db_user): <p>The database user name. This parameter is required when connecting to a cluster and authenticating using temporary credentials. </p>
    ///   - [`database(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::database) / [`set_database(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_database): <p>The name of the database. This parameter is required when authenticating using either Secrets Manager or temporary credentials. </p>
    ///   - [`with_event(bool)`](crate::client::fluent_builders::ExecuteStatement::with_event) / [`set_with_event(Option<bool>)`](crate::client::fluent_builders::ExecuteStatement::set_with_event): <p>A value that indicates whether to send an event to the Amazon EventBridge event bus after the SQL statement runs. </p>
    ///   - [`statement_name(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::statement_name) / [`set_statement_name(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_statement_name): <p>The name of the SQL statement. You can name the SQL statement when you create it to identify the query. </p>
    ///   - [`parameters(Vec<SqlParameter>)`](crate::client::fluent_builders::ExecuteStatement::parameters) / [`set_parameters(Option<Vec<SqlParameter>>)`](crate::client::fluent_builders::ExecuteStatement::set_parameters): <p>The parameters for the SQL statement.</p>
    ///   - [`workgroup_name(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::workgroup_name) / [`set_workgroup_name(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_workgroup_name): <p>The serverless workgroup name. This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::ExecuteStatement::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::ExecuteStatement::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
                            /// - On success, responds with [`ExecuteStatementOutput`](crate::output::ExecuteStatementOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::ExecuteStatementOutput::id): <p>The identifier of the SQL statement whose results are to be fetched. This value is a universally unique identifier (UUID) generated by Amazon Redshift Data API. </p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::ExecuteStatementOutput::created_at): <p>The date and time (UTC) the statement was created. </p>
    ///   - [`cluster_identifier(Option<String>)`](crate::output::ExecuteStatementOutput::cluster_identifier): <p>The cluster identifier. This element is not returned when connecting to a serverless workgroup. </p>
    ///   - [`db_user(Option<String>)`](crate::output::ExecuteStatementOutput::db_user): <p>The database user name.</p>
    ///   - [`database(Option<String>)`](crate::output::ExecuteStatementOutput::database): <p>The name of the database.</p>
    ///   - [`secret_arn(Option<String>)`](crate::output::ExecuteStatementOutput::secret_arn): <p>The name or ARN of the secret that enables access to the database. </p>
    ///   - [`workgroup_name(Option<String>)`](crate::output::ExecuteStatementOutput::workgroup_name): <p>The serverless workgroup name. This element is not returned when connecting to a provisioned cluster.</p>
                            /// - On failure, responds with [`SdkError<ExecuteStatementError>`](crate::error::ExecuteStatementError)
    pub fn execute_statement(&self) -> crate::client::fluent_builders::ExecuteStatement {
                                crate::client::fluent_builders::ExecuteStatement::new(self.handle.clone())
                            }
}

