// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDBClusterSnapshotAttributes`](crate::client::fluent_builders::DescribeDBClusterSnapshotAttributes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_cluster_snapshot_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeDBClusterSnapshotAttributes::db_cluster_snapshot_identifier) / [`set_db_cluster_snapshot_identifier(Option<String>)`](crate::client::fluent_builders::DescribeDBClusterSnapshotAttributes::set_db_cluster_snapshot_identifier): <p>The identifier for the DB cluster snapshot to describe the attributes for.</p>
                            /// - On success, responds with [`DescribeDbClusterSnapshotAttributesOutput`](crate::output::DescribeDbClusterSnapshotAttributesOutput) with field(s):
    ///   - [`db_cluster_snapshot_attributes_result(Option<DbClusterSnapshotAttributesResult>)`](crate::output::DescribeDbClusterSnapshotAttributesOutput::db_cluster_snapshot_attributes_result): <p>Contains the results of a successful call to the <code>DescribeDBClusterSnapshotAttributes</code> API action.</p>  <p>Manual DB cluster snapshot attributes are used to authorize other Amazon Web Services accounts to copy or restore a manual DB cluster snapshot. For more information, see the <code>ModifyDBClusterSnapshotAttribute</code> API action.</p>
                            /// - On failure, responds with [`SdkError<DescribeDBClusterSnapshotAttributesError>`](crate::error::DescribeDBClusterSnapshotAttributesError)
    pub fn describe_db_cluster_snapshot_attributes(&self) -> crate::client::fluent_builders::DescribeDBClusterSnapshotAttributes {
                                crate::client::fluent_builders::DescribeDBClusterSnapshotAttributes::new(self.handle.clone())
                            }
}

