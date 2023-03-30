// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteReportPlan`](crate::client::fluent_builders::DeleteReportPlan) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`report_plan_name(impl Into<String>)`](crate::client::fluent_builders::DeleteReportPlan::report_plan_name) / [`set_report_plan_name(Option<String>)`](crate::client::fluent_builders::DeleteReportPlan::set_report_plan_name): <p>The unique name of a report plan.</p>
                            /// - On success, responds with [`DeleteReportPlanOutput`](crate::output::DeleteReportPlanOutput)
                            /// - On failure, responds with [`SdkError<DeleteReportPlanError>`](crate::error::DeleteReportPlanError)
    pub fn delete_report_plan(&self) -> crate::client::fluent_builders::DeleteReportPlan {
                                crate::client::fluent_builders::DeleteReportPlan::new(self.handle.clone())
                            }
}

