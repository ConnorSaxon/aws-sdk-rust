// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListBulkDeploymentDetailedReports`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bulk_deployment_id(impl Into<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::bulk_deployment_id) / [`set_bulk_deployment_id(Option<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::set_bulk_deployment_id): The ID of the bulk deployment.
    ///   - [`max_results(impl Into<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::max_results) / [`set_max_results(Option<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::set_max_results): The maximum number of results to be returned per request.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListBulkDeploymentDetailedReports::set_next_token): The token for the next set of results, or ''null'' if there are no additional results.
                            /// - On success, responds with [`ListBulkDeploymentDetailedReportsOutput`](crate::output::ListBulkDeploymentDetailedReportsOutput) with field(s):
    ///   - [`deployments(Option<Vec<BulkDeploymentResult>>)`](crate::output::ListBulkDeploymentDetailedReportsOutput::deployments): A list of the individual group deployments in the bulk deployment operation.
    ///   - [`next_token(Option<String>)`](crate::output::ListBulkDeploymentDetailedReportsOutput::next_token): The token for the next set of results, or ''null'' if there are no additional results.
                            /// - On failure, responds with [`SdkError<ListBulkDeploymentDetailedReportsError>`](crate::error::ListBulkDeploymentDetailedReportsError)
    pub fn list_bulk_deployment_detailed_reports(&self) -> crate::client::fluent_builders::ListBulkDeploymentDetailedReports {
                                crate::client::fluent_builders::ListBulkDeploymentDetailedReports::new(self.handle.clone())
                            }
}

