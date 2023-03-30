// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReplicationGroups`](crate::client::fluent_builders::DescribeReplicationGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeReplicationGroups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`replication_group_id(impl Into<String>)`](crate::client::fluent_builders::DescribeReplicationGroups::replication_group_id) / [`set_replication_group_id(Option<String>)`](crate::client::fluent_builders::DescribeReplicationGroups::set_replication_group_id): <p>The identifier for the replication group to be described. This parameter is not case sensitive.</p>  <p>If you do not specify this parameter, information about all replication groups is returned.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeReplicationGroups::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeReplicationGroups::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: minimum 20; maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeReplicationGroups::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeReplicationGroups::set_marker): <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeReplicationGroupsOutput`](crate::output::DescribeReplicationGroupsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeReplicationGroupsOutput::marker): <p>Provides an identifier to allow retrieval of paginated results.</p>
    ///   - [`replication_groups(Option<Vec<ReplicationGroup>>)`](crate::output::DescribeReplicationGroupsOutput::replication_groups): <p>A list of replication groups. Each item in the list contains detailed information about one replication group.</p>
                            /// - On failure, responds with [`SdkError<DescribeReplicationGroupsError>`](crate::error::DescribeReplicationGroupsError)
    pub fn describe_replication_groups(&self) -> crate::client::fluent_builders::DescribeReplicationGroups {
                                crate::client::fluent_builders::DescribeReplicationGroups::new(self.handle.clone())
                            }
}

