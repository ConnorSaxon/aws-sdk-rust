// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeVirtualCluster`](crate::client::fluent_builders::DescribeVirtualCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DescribeVirtualCluster::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DescribeVirtualCluster::set_id): <p>The ID of the virtual cluster that will be described.</p>
                            /// - On success, responds with [`DescribeVirtualClusterOutput`](crate::output::DescribeVirtualClusterOutput) with field(s):
    ///   - [`virtual_cluster(Option<VirtualCluster>)`](crate::output::DescribeVirtualClusterOutput::virtual_cluster): <p>This output displays information about the specified virtual cluster.</p>
                            /// - On failure, responds with [`SdkError<DescribeVirtualClusterError>`](crate::error::DescribeVirtualClusterError)
    pub fn describe_virtual_cluster(&self) -> crate::client::fluent_builders::DescribeVirtualCluster {
                                crate::client::fluent_builders::DescribeVirtualCluster::new(self.handle.clone())
                            }
}

