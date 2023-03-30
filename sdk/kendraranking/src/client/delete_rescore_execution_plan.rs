// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRescoreExecutionPlan`](crate::client::fluent_builders::DeleteRescoreExecutionPlan) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteRescoreExecutionPlan::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteRescoreExecutionPlan::set_id): <p>The identifier of the rescore execution plan that you want to delete.</p>
                            /// - On success, responds with [`DeleteRescoreExecutionPlanOutput`](crate::output::DeleteRescoreExecutionPlanOutput)
                            /// - On failure, responds with [`SdkError<DeleteRescoreExecutionPlanError>`](crate::error::DeleteRescoreExecutionPlanError)
    pub fn delete_rescore_execution_plan(&self) -> crate::client::fluent_builders::DeleteRescoreExecutionPlan {
                                crate::client::fluent_builders::DeleteRescoreExecutionPlan::new(self.handle.clone())
                            }
}

