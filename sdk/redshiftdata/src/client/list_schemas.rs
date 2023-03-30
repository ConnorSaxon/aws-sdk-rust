// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSchemas`](crate::client::fluent_builders::ListSchemas) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSchemas::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_cluster_identifier): <p>The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials. </p>
    ///   - [`secret_arn(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::secret_arn) / [`set_secret_arn(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_secret_arn): <p>The name or ARN of the secret that enables access to the database. This parameter is required when authenticating using Secrets Manager. </p>
    ///   - [`db_user(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::db_user) / [`set_db_user(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_db_user): <p>The database user name. This parameter is required when connecting to a cluster and authenticating using temporary credentials. </p>
    ///   - [`database(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::database) / [`set_database(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_database): <p>The name of the database that contains the schemas to list. If <code>ConnectedDatabase</code> is not specified, this is also the database to connect to with your authentication credentials.</p>
    ///   - [`connected_database(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::connected_database) / [`set_connected_database(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_connected_database): <p>A database name. The connected database is specified when you connect with your authentication credentials. </p>
    ///   - [`schema_pattern(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::schema_pattern) / [`set_schema_pattern(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_schema_pattern): <p>A pattern to filter results by schema name. Within a schema pattern, "%" means match any substring of 0 or more characters and "_" means match any one character. Only schema name entries matching the search pattern are returned. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_next_token): <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned NextToken value in the next NextToken parameter and retrying the command. If the NextToken field is empty, all response records have been retrieved for the request. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSchemas::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListSchemas::set_max_results): <p>The maximum number of schemas to return in the response. If more schemas exist than fit in one response, then <code>NextToken</code> is returned to page through the results. </p>
    ///   - [`workgroup_name(impl Into<String>)`](crate::client::fluent_builders::ListSchemas::workgroup_name) / [`set_workgroup_name(Option<String>)`](crate::client::fluent_builders::ListSchemas::set_workgroup_name): <p>The serverless workgroup name. This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.</p>
                            /// - On success, responds with [`ListSchemasOutput`](crate::output::ListSchemasOutput) with field(s):
    ///   - [`schemas(Option<Vec<String>>)`](crate::output::ListSchemasOutput::schemas): <p>The schemas that match the request pattern. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSchemasOutput::next_token): <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned NextToken value in the next NextToken parameter and retrying the command. If the NextToken field is empty, all response records have been retrieved for the request. </p>
                            /// - On failure, responds with [`SdkError<ListSchemasError>`](crate::error::ListSchemasError)
    pub fn list_schemas(&self) -> crate::client::fluent_builders::ListSchemas {
                                crate::client::fluent_builders::ListSchemas::new(self.handle.clone())
                            }
}

