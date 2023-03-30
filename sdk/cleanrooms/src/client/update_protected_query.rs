// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateProtectedQuery`](crate::client::fluent_builders::UpdateProtectedQuery) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::client::fluent_builders::UpdateProtectedQuery::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::client::fluent_builders::UpdateProtectedQuery::set_membership_identifier): <p>The identifier for a member of a protected query instance.</p>
    ///   - [`protected_query_identifier(impl Into<String>)`](crate::client::fluent_builders::UpdateProtectedQuery::protected_query_identifier) / [`set_protected_query_identifier(Option<String>)`](crate::client::fluent_builders::UpdateProtectedQuery::set_protected_query_identifier): <p>The identifier for a protected query instance.</p>
    ///   - [`target_status(TargetProtectedQueryStatus)`](crate::client::fluent_builders::UpdateProtectedQuery::target_status) / [`set_target_status(Option<TargetProtectedQueryStatus>)`](crate::client::fluent_builders::UpdateProtectedQuery::set_target_status): <p>The target status of a query. Used to update the execution status of a currently running query.</p>
                            /// - On success, responds with [`UpdateProtectedQueryOutput`](crate::output::UpdateProtectedQueryOutput) with field(s):
    ///   - [`protected_query(Option<ProtectedQuery>)`](crate::output::UpdateProtectedQueryOutput::protected_query): <p>The protected query output.</p>
                            /// - On failure, responds with [`SdkError<UpdateProtectedQueryError>`](crate::error::UpdateProtectedQueryError)
    pub fn update_protected_query(&self) -> crate::client::fluent_builders::UpdateProtectedQuery {
                                crate::client::fluent_builders::UpdateProtectedQuery::new(self.handle.clone())
                            }
}

