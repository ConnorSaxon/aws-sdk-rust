// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDBInstanceAutomatedBackups`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dbi_resource_id(impl Into<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::dbi_resource_id) / [`set_dbi_resource_id(Option<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_dbi_resource_id): <p>The resource ID of the DB instance that is the source of the automated backup. This parameter isn't case-sensitive.</p>
    ///   - [`db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_db_instance_identifier): <p>(Optional) The user-supplied instance identifier. If this parameter is specified, it must match the identifier of an existing DB instance. It returns information from the specific DB instance' automated backup. This parameter isn't case-sensitive.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_filters): <p>A filter that specifies which resources to return based on status.</p>  <p>Supported filters are the following:</p>  <ul>   <li> <p> <code>status</code> </p>    <ul>     <li> <p> <code>active</code> - automated backups for current instances</p> </li>     <li> <p> <code>retained</code> - automated backups for deleted instances and after backup replication is stopped</p> </li>     <li> <p> <code>creating</code> - automated backups that are waiting for the first automated snapshot to be available</p> </li>    </ul> </li>   <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and Amazon Resource Names (ARNs). The results list includes only information about the DB instance automated backups identified by these ARNs.</p> </li>   <li> <p> <code>dbi-resource-id</code> - Accepts DB resource identifiers and Amazon Resource Names (ARNs). The results list includes only information about the DB instance resources identified by these ARNs.</p> </li>  </ul>  <p>Returns all resources by default. The status for each resource is specified in the response.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_marker): <p>The pagination token provided in the previous request. If this parameter is specified the response includes only records beyond the marker, up to <code>MaxRecords</code>.</p>
    ///   - [`db_instance_automated_backups_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::db_instance_automated_backups_arn) / [`set_db_instance_automated_backups_arn(Option<String>)`](crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::set_db_instance_automated_backups_arn): <p>The Amazon Resource Name (ARN) of the replicated automated backups, for example, <code>arn:aws:rds:us-east-1:123456789012:auto-backup:ab-L2IJCEXJP7XQ7HOJ4SIEXAMPLE</code>.</p>  <p>This setting doesn't apply to RDS Custom.</p>
                            /// - On success, responds with [`DescribeDbInstanceAutomatedBackupsOutput`](crate::output::DescribeDbInstanceAutomatedBackupsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeDbInstanceAutomatedBackupsOutput::marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`db_instance_automated_backups(Option<Vec<DbInstanceAutomatedBackup>>)`](crate::output::DescribeDbInstanceAutomatedBackupsOutput::db_instance_automated_backups): <p>A list of <code>DBInstanceAutomatedBackup</code> instances.</p>
                            /// - On failure, responds with [`SdkError<DescribeDBInstanceAutomatedBackupsError>`](crate::error::DescribeDBInstanceAutomatedBackupsError)
    pub fn describe_db_instance_automated_backups(&self) -> crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups {
                                crate::client::fluent_builders::DescribeDBInstanceAutomatedBackups::new(self.handle.clone())
                            }
}

