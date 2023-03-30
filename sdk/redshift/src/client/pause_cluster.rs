// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PauseCluster`](crate::client::fluent_builders::PauseCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::PauseCluster::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::PauseCluster::set_cluster_identifier): <p>The identifier of the cluster to be paused.</p>
                            /// - On success, responds with [`PauseClusterOutput`](crate::output::PauseClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::PauseClusterOutput::cluster): <p>Describes a cluster.</p>
                            /// - On failure, responds with [`SdkError<PauseClusterError>`](crate::error::PauseClusterError)
    pub fn pause_cluster(&self) -> crate::client::fluent_builders::PauseCluster {
                                crate::client::fluent_builders::PauseCluster::new(self.handle.clone())
                            }
}

