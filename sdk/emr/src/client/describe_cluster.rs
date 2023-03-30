// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeCluster`](crate::client::fluent_builders::DescribeCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl Into<String>)`](crate::client::fluent_builders::DescribeCluster::cluster_id) / [`set_cluster_id(Option<String>)`](crate::client::fluent_builders::DescribeCluster::set_cluster_id): <p>The identifier of the cluster to describe.</p>
                            /// - On success, responds with [`DescribeClusterOutput`](crate::output::DescribeClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::DescribeClusterOutput::cluster): <p>This output contains the details for the requested cluster.</p>
                            /// - On failure, responds with [`SdkError<DescribeClusterError>`](crate::error::DescribeClusterError)
    pub fn describe_cluster(&self) -> crate::client::fluent_builders::DescribeCluster {
                                crate::client::fluent_builders::DescribeCluster::new(self.handle.clone())
                            }
}

