// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDashboard`](crate::client::fluent_builders::DeleteDashboard) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dashboard_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDashboard::dashboard_id) / [`set_dashboard_id(Option<String>)`](crate::client::fluent_builders::DeleteDashboard::set_dashboard_id): <p>The ID of the dashboard to delete.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::DeleteDashboard::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::DeleteDashboard::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
                            /// - On success, responds with [`DeleteDashboardOutput`](crate::output::DeleteDashboardOutput)
                            /// - On failure, responds with [`SdkError<DeleteDashboardError>`](crate::error::DeleteDashboardError)
    pub fn delete_dashboard(&self) -> crate::client::fluent_builders::DeleteDashboard {
                                crate::client::fluent_builders::DeleteDashboard::new(self.handle.clone())
                            }
}

