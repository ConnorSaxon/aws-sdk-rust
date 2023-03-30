// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePreparedStatement`](crate::client::fluent_builders::DeletePreparedStatement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`statement_name(impl Into<String>)`](crate::client::fluent_builders::DeletePreparedStatement::statement_name) / [`set_statement_name(Option<String>)`](crate::client::fluent_builders::DeletePreparedStatement::set_statement_name): <p>The name of the prepared statement to delete.</p>
    ///   - [`work_group(impl Into<String>)`](crate::client::fluent_builders::DeletePreparedStatement::work_group) / [`set_work_group(Option<String>)`](crate::client::fluent_builders::DeletePreparedStatement::set_work_group): <p>The workgroup to which the statement to be deleted belongs.</p>
                            /// - On success, responds with [`DeletePreparedStatementOutput`](crate::output::DeletePreparedStatementOutput)
                            /// - On failure, responds with [`SdkError<DeletePreparedStatementError>`](crate::error::DeletePreparedStatementError)
    pub fn delete_prepared_statement(&self) -> crate::client::fluent_builders::DeletePreparedStatement {
                                crate::client::fluent_builders::DeletePreparedStatement::new(self.handle.clone())
                            }
}

