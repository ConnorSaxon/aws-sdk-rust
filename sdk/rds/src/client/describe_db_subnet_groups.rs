// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDBSubnetGroups`](crate::client::fluent_builders::DescribeDBSubnetGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeDBSubnetGroups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::set_db_subnet_group_name): <p>The name of the DB subnet group to return details for.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::set_filters): <p>This parameter isn't currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeDBSubnetGroups::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeDBSubnetGroups::set_marker): <p>An optional pagination token provided by a previous DescribeDBSubnetGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeDbSubnetGroupsOutput`](crate::output::DescribeDbSubnetGroupsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeDbSubnetGroupsOutput::marker): <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    ///   - [`db_subnet_groups(Option<Vec<DbSubnetGroup>>)`](crate::output::DescribeDbSubnetGroupsOutput::db_subnet_groups): <p>A list of <code>DBSubnetGroup</code> instances.</p>
                            /// - On failure, responds with [`SdkError<DescribeDBSubnetGroupsError>`](crate::error::DescribeDBSubnetGroupsError)
    pub fn describe_db_subnet_groups(&self) -> crate::client::fluent_builders::DescribeDBSubnetGroups {
                                crate::client::fluent_builders::DescribeDBSubnetGroups::new(self.handle.clone())
                            }
}

