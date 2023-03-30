// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RebootCacheCluster`](crate::client::fluent_builders::RebootCacheCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cache_cluster_id(impl Into<String>)`](crate::client::fluent_builders::RebootCacheCluster::cache_cluster_id) / [`set_cache_cluster_id(Option<String>)`](crate::client::fluent_builders::RebootCacheCluster::set_cache_cluster_id): <p>The cluster identifier. This parameter is stored as a lowercase string.</p>
    ///   - [`cache_node_ids_to_reboot(Vec<String>)`](crate::client::fluent_builders::RebootCacheCluster::cache_node_ids_to_reboot) / [`set_cache_node_ids_to_reboot(Option<Vec<String>>)`](crate::client::fluent_builders::RebootCacheCluster::set_cache_node_ids_to_reboot): <p>A list of cache node IDs to reboot. A node ID is a numeric identifier (0001, 0002, etc.). To reboot an entire cluster, specify all of the cache node IDs.</p>
                            /// - On success, responds with [`RebootCacheClusterOutput`](crate::output::RebootCacheClusterOutput) with field(s):
    ///   - [`cache_cluster(Option<CacheCluster>)`](crate::output::RebootCacheClusterOutput::cache_cluster): <p>Contains all of the attributes of a specific cluster.</p>
                            /// - On failure, responds with [`SdkError<RebootCacheClusterError>`](crate::error::RebootCacheClusterError)
    pub fn reboot_cache_cluster(&self) -> crate::client::fluent_builders::RebootCacheCluster {
                                crate::client::fluent_builders::RebootCacheCluster::new(self.handle.clone())
                            }
}

