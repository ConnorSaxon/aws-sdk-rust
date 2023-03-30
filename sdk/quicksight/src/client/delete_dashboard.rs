// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDashboard`](crate::client::fluent_builders::DeleteDashboard) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDashboard::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::DeleteDashboard::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the dashboard that you're deleting.</p>
    ///   - [`dashboard_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDashboard::dashboard_id) / [`set_dashboard_id(Option<String>)`](crate::client::fluent_builders::DeleteDashboard::set_dashboard_id): <p>The ID for the dashboard.</p>
    ///   - [`version_number(i64)`](crate::client::fluent_builders::DeleteDashboard::version_number) / [`set_version_number(Option<i64>)`](crate::client::fluent_builders::DeleteDashboard::set_version_number): <p>The version number of the dashboard. If the version number property is provided, only the specified version of the dashboard is deleted.</p>
                            /// - On success, responds with [`DeleteDashboardOutput`](crate::output::DeleteDashboardOutput) with field(s):
    ///   - [`status(i32)`](crate::output::DeleteDashboardOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`arn(Option<String>)`](crate::output::DeleteDashboardOutput::arn): <p>The Secure Socket Layer (SSL) properties that apply for the resource.</p>
    ///   - [`dashboard_id(Option<String>)`](crate::output::DeleteDashboardOutput::dashboard_id): <p>The ID of the dashboard.</p>
    ///   - [`request_id(Option<String>)`](crate::output::DeleteDashboardOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<DeleteDashboardError>`](crate::error::DeleteDashboardError)
    pub fn delete_dashboard(&self) -> crate::client::fluent_builders::DeleteDashboard {
                                crate::client::fluent_builders::DeleteDashboard::new(self.handle.clone())
                            }
}

