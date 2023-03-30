// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePendingMaintenanceActions`](crate::client::fluent_builders::DescribePendingMaintenanceActions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribePendingMaintenanceActions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::resource_identifier) / [`set_resource_identifier(Option<String>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::set_resource_identifier): <p>The ARN of a resource to return pending maintenance actions for.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::set_filters): <p>A filter that specifies one or more resources to return pending maintenance actions for.</p>  <p>Supported filters:</p>  <ul>   <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include pending maintenance actions for the DB clusters identified by these ARNs.</p> </li>   <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance ARNs. The results list will only include pending maintenance actions for the DB instances identified by these ARNs.</p> </li>  </ul>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::set_marker): <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribePendingMaintenanceActions::set_max_records): <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
                            /// - On success, responds with [`DescribePendingMaintenanceActionsOutput`](crate::output::DescribePendingMaintenanceActionsOutput) with field(s):
    ///   - [`pending_maintenance_actions(Option<Vec<ResourcePendingMaintenanceActions>>)`](crate::output::DescribePendingMaintenanceActionsOutput::pending_maintenance_actions): <p>A list of the pending maintenance actions for the resource.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribePendingMaintenanceActionsOutput::marker): <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>.</p>
                            /// - On failure, responds with [`SdkError<DescribePendingMaintenanceActionsError>`](crate::error::DescribePendingMaintenanceActionsError)
    pub fn describe_pending_maintenance_actions(&self) -> crate::client::fluent_builders::DescribePendingMaintenanceActions {
                                crate::client::fluent_builders::DescribePendingMaintenanceActions::new(self.handle.clone())
                            }
}

