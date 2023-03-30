// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRelationalDatabaseParameters`](crate::client::fluent_builders::GetRelationalDatabaseParameters) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`relational_database_name(impl Into<String>)`](crate::client::fluent_builders::GetRelationalDatabaseParameters::relational_database_name) / [`set_relational_database_name(Option<String>)`](crate::client::fluent_builders::GetRelationalDatabaseParameters::set_relational_database_name): <p>The name of your database for which to get parameters.</p>
    ///   - [`page_token(impl Into<String>)`](crate::client::fluent_builders::GetRelationalDatabaseParameters::page_token) / [`set_page_token(Option<String>)`](crate::client::fluent_builders::GetRelationalDatabaseParameters::set_page_token): <p>The token to advance to the next page of results from your request.</p>  <p>To get a page token, perform an initial <code>GetRelationalDatabaseParameters</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
                            /// - On success, responds with [`GetRelationalDatabaseParametersOutput`](crate::output::GetRelationalDatabaseParametersOutput) with field(s):
    ///   - [`parameters(Option<Vec<RelationalDatabaseParameter>>)`](crate::output::GetRelationalDatabaseParametersOutput::parameters): <p>An object describing the result of your get relational database parameters request.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::GetRelationalDatabaseParametersOutput::next_page_token): <p>The token to advance to the next page of results from your request.</p>  <p>A next page token is not returned if there are no more results to display.</p>  <p>To get the next page of results, perform another <code>GetRelationalDatabaseParameters</code> request and specify the next page token using the <code>pageToken</code> parameter.</p>
                            /// - On failure, responds with [`SdkError<GetRelationalDatabaseParametersError>`](crate::error::GetRelationalDatabaseParametersError)
    pub fn get_relational_database_parameters(&self) -> crate::client::fluent_builders::GetRelationalDatabaseParameters {
                                crate::client::fluent_builders::GetRelationalDatabaseParameters::new(self.handle.clone())
                            }
}

