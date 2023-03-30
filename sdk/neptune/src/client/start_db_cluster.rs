// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartDBCluster`](crate::client::fluent_builders::StartDBCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::StartDBCluster::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::client::fluent_builders::StartDBCluster::set_db_cluster_identifier): <p>The DB cluster identifier of the Neptune DB cluster to be started. This parameter is stored as a lowercase string.</p>
                            /// - On success, responds with [`StartDbClusterOutput`](crate::output::StartDbClusterOutput) with field(s):
    ///   - [`db_cluster(Option<DbCluster>)`](crate::output::StartDbClusterOutput::db_cluster): <p>Contains the details of an Amazon Neptune DB cluster.</p>  <p>This data type is used as a response element in the <code>DescribeDBClusters</code> action.</p>
                            /// - On failure, responds with [`SdkError<StartDBClusterError>`](crate::error::StartDBClusterError)
    pub fn start_db_cluster(&self) -> crate::client::fluent_builders::StartDBCluster {
                                crate::client::fluent_builders::StartDBCluster::new(self.handle.clone())
                            }
}

