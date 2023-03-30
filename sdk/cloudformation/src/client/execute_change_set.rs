// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ExecuteChangeSet`](crate::client::fluent_builders::ExecuteChangeSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`change_set_name(impl Into<String>)`](crate::client::fluent_builders::ExecuteChangeSet::change_set_name) / [`set_change_set_name(Option<String>)`](crate::client::fluent_builders::ExecuteChangeSet::set_change_set_name): <p>The name or Amazon Resource Name (ARN) of the change set that you want use to update the specified stack.</p>
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::ExecuteChangeSet::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::ExecuteChangeSet::set_stack_name): <p>If you specified the name of a change set, specify the stack name or Amazon Resource Name (ARN) that's associated with the change set you want to execute.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::ExecuteChangeSet::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::ExecuteChangeSet::set_client_request_token): <p>A unique identifier for this <code>ExecuteChangeSet</code> request. Specify this token if you plan to retry requests so that CloudFormation knows that you're not attempting to execute a change set to update a stack with the same name. You might retry <code>ExecuteChangeSet</code> requests to ensure that CloudFormation successfully received them.</p>
    ///   - [`disable_rollback(bool)`](crate::client::fluent_builders::ExecuteChangeSet::disable_rollback) / [`set_disable_rollback(Option<bool>)`](crate::client::fluent_builders::ExecuteChangeSet::set_disable_rollback): <p>Preserves the state of previously provisioned resources when an operation fails.</p>  <p>Default: <code>True</code> </p>
                            /// - On success, responds with [`ExecuteChangeSetOutput`](crate::output::ExecuteChangeSetOutput)
                            /// - On failure, responds with [`SdkError<ExecuteChangeSetError>`](crate::error::ExecuteChangeSetError)
    pub fn execute_change_set(&self) -> crate::client::fluent_builders::ExecuteChangeSet {
                                crate::client::fluent_builders::ExecuteChangeSet::new(self.handle.clone())
                            }
}

