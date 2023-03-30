// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteCluster`](crate::client::fluent_builders::DeleteCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteCluster::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::client::fluent_builders::DeleteCluster::set_cluster_arn): <p>The arn of the Elastic DocumentDB cluster that is to be deleted.</p>
                            /// - On success, responds with [`DeleteClusterOutput`](crate::output::DeleteClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::DeleteClusterOutput::cluster): <p>Returns information about the newly deleted Elastic DocumentDB cluster.</p>
                            /// - On failure, responds with [`SdkError<DeleteClusterError>`](crate::error::DeleteClusterError)
    pub fn delete_cluster(&self) -> crate::client::fluent_builders::DeleteCluster {
                                crate::client::fluent_builders::DeleteCluster::new(self.handle.clone())
                            }
}

