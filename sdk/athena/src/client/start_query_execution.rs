// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartQueryExecution`](crate::client::fluent_builders::StartQueryExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`query_string(impl Into<String>)`](crate::client::fluent_builders::StartQueryExecution::query_string) / [`set_query_string(Option<String>)`](crate::client::fluent_builders::StartQueryExecution::set_query_string): <p>The SQL query statements to be executed.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::StartQueryExecution::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::StartQueryExecution::set_client_request_token): <p>A unique case-sensitive string used to ensure the request to create the query is idempotent (executes only once). If another <code>StartQueryExecution</code> request is received, the same response is returned and another query is not created. If a parameter has changed, for example, the <code>QueryString</code>, an error is returned.</p> <important>   <p>This token is listed as not required because Amazon Web Services SDKs (for example the Amazon Web Services SDK for Java) auto-generate the token for users. If you are not using the Amazon Web Services SDK or the Amazon Web Services CLI, you must provide this token or the action will fail.</p>  </important>
    ///   - [`query_execution_context(QueryExecutionContext)`](crate::client::fluent_builders::StartQueryExecution::query_execution_context) / [`set_query_execution_context(Option<QueryExecutionContext>)`](crate::client::fluent_builders::StartQueryExecution::set_query_execution_context): <p>The database within which the query executes.</p>
    ///   - [`result_configuration(ResultConfiguration)`](crate::client::fluent_builders::StartQueryExecution::result_configuration) / [`set_result_configuration(Option<ResultConfiguration>)`](crate::client::fluent_builders::StartQueryExecution::set_result_configuration): <p>Specifies information about where and how to save the results of the query execution. If the query runs in a workgroup, then workgroup's settings may override query settings. This affects the query results location. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <code>WorkGroupConfiguration$EnforceWorkGroupConfiguration</code>.</p>
    ///   - [`work_group(impl Into<String>)`](crate::client::fluent_builders::StartQueryExecution::work_group) / [`set_work_group(Option<String>)`](crate::client::fluent_builders::StartQueryExecution::set_work_group): <p>The name of the workgroup in which the query is being started.</p>
    ///   - [`execution_parameters(Vec<String>)`](crate::client::fluent_builders::StartQueryExecution::execution_parameters) / [`set_execution_parameters(Option<Vec<String>>)`](crate::client::fluent_builders::StartQueryExecution::set_execution_parameters): <p>A list of values for the parameters in a query. The values are applied sequentially to the parameters in the query in the order in which the parameters occur.</p>
    ///   - [`result_reuse_configuration(ResultReuseConfiguration)`](crate::client::fluent_builders::StartQueryExecution::result_reuse_configuration) / [`set_result_reuse_configuration(Option<ResultReuseConfiguration>)`](crate::client::fluent_builders::StartQueryExecution::set_result_reuse_configuration): <p>Specifies the query result reuse behavior for the query.</p>
                            /// - On success, responds with [`StartQueryExecutionOutput`](crate::output::StartQueryExecutionOutput) with field(s):
    ///   - [`query_execution_id(Option<String>)`](crate::output::StartQueryExecutionOutput::query_execution_id): <p>The unique ID of the query that ran as a result of this request.</p>
                            /// - On failure, responds with [`SdkError<StartQueryExecutionError>`](crate::error::StartQueryExecutionError)
    pub fn start_query_execution(&self) -> crate::client::fluent_builders::StartQueryExecution {
                                crate::client::fluent_builders::StartQueryExecution::new(self.handle.clone())
                            }
}

