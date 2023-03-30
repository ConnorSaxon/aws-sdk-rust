// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelStatement`](crate::client::fluent_builders::CancelStatement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::CancelStatement::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::CancelStatement::set_id): <p>The identifier of the SQL statement to cancel. This value is a universally unique identifier (UUID) generated by Amazon Redshift Data API. This identifier is returned by <code>BatchExecuteStatment</code>, <code>ExecuteStatment</code>, and <code>ListStatements</code>. </p>
                            /// - On success, responds with [`CancelStatementOutput`](crate::output::CancelStatementOutput) with field(s):
    ///   - [`status(Option<bool>)`](crate::output::CancelStatementOutput::status): <p>A value that indicates whether the cancel statement succeeded (true). </p>
                            /// - On failure, responds with [`SdkError<CancelStatementError>`](crate::error::CancelStatementError)
    pub fn cancel_statement(&self) -> crate::client::fluent_builders::CancelStatement {
                                crate::client::fluent_builders::CancelStatement::new(self.handle.clone())
                            }
}

