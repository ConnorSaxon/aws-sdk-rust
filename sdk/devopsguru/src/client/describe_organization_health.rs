// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOrganizationHealth`](crate::client::fluent_builders::DescribeOrganizationHealth) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::DescribeOrganizationHealth::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeOrganizationHealth::set_account_ids): <p>The ID of the Amazon Web Services account.</p>
    ///   - [`organizational_unit_ids(Vec<String>)`](crate::client::fluent_builders::DescribeOrganizationHealth::organizational_unit_ids) / [`set_organizational_unit_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeOrganizationHealth::set_organizational_unit_ids): <p>The ID of the organizational unit.</p>
                            /// - On success, responds with [`DescribeOrganizationHealthOutput`](crate::output::DescribeOrganizationHealthOutput) with field(s):
    ///   - [`open_reactive_insights(i32)`](crate::output::DescribeOrganizationHealthOutput::open_reactive_insights): <p>An integer that specifies the number of open reactive insights in your Amazon Web Services account.</p>
    ///   - [`open_proactive_insights(i32)`](crate::output::DescribeOrganizationHealthOutput::open_proactive_insights): <p>An integer that specifies the number of open proactive insights in your Amazon Web Services account.</p>
    ///   - [`metrics_analyzed(i32)`](crate::output::DescribeOrganizationHealthOutput::metrics_analyzed): <p>An integer that specifies the number of metrics that have been analyzed in your organization.</p>
    ///   - [`resource_hours(Option<i64>)`](crate::output::DescribeOrganizationHealthOutput::resource_hours): <p>The number of Amazon DevOps Guru resource analysis hours billed to the current Amazon Web Services account in the last hour. </p>
                            /// - On failure, responds with [`SdkError<DescribeOrganizationHealthError>`](crate::error::DescribeOrganizationHealthError)
    pub fn describe_organization_health(&self) -> crate::client::fluent_builders::DescribeOrganizationHealth {
                                crate::client::fluent_builders::DescribeOrganizationHealth::new(self.handle.clone())
                            }
}

