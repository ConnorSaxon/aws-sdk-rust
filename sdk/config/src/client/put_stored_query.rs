// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutStoredQuery`](crate::client::fluent_builders::PutStoredQuery) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stored_query(StoredQuery)`](crate::client::fluent_builders::PutStoredQuery::stored_query) / [`set_stored_query(Option<StoredQuery>)`](crate::client::fluent_builders::PutStoredQuery::set_stored_query): <p>A list of <code>StoredQuery</code> objects. The mandatory fields are <code>QueryName</code> and <code>Expression</code>.</p> <note>   <p>When you are creating a query, you must provide a query name and an expression. When you are updating a query, you must provide a query name but updating the description is optional.</p>  </note>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::PutStoredQuery::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::PutStoredQuery::set_tags): <p>A list of <code>Tags</code> object.</p>
                            /// - On success, responds with [`PutStoredQueryOutput`](crate::output::PutStoredQueryOutput) with field(s):
    ///   - [`query_arn(Option<String>)`](crate::output::PutStoredQueryOutput::query_arn): <p>Amazon Resource Name (ARN) of the query. For example, arn:partition:service:region:account-id:resource-type/resource-name/resource-id.</p>
                            /// - On failure, responds with [`SdkError<PutStoredQueryError>`](crate::error::PutStoredQueryError)
    pub fn put_stored_query(&self) -> crate::client::fluent_builders::PutStoredQuery {
                                crate::client::fluent_builders::PutStoredQuery::new(self.handle.clone())
                            }
}

