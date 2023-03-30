// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAllowedNodeTypeModifications`](crate::client::fluent_builders::ListAllowedNodeTypeModifications) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cache_cluster_id(impl Into<String>)`](crate::client::fluent_builders::ListAllowedNodeTypeModifications::cache_cluster_id) / [`set_cache_cluster_id(Option<String>)`](crate::client::fluent_builders::ListAllowedNodeTypeModifications::set_cache_cluster_id): <p>The name of the cluster you want to scale up to a larger node instanced type. ElastiCache uses the cluster id to identify the current node type of this cluster and from that to create a list of node types you can scale up to.</p> <important>   <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p>  </important>
    ///   - [`replication_group_id(impl Into<String>)`](crate::client::fluent_builders::ListAllowedNodeTypeModifications::replication_group_id) / [`set_replication_group_id(Option<String>)`](crate::client::fluent_builders::ListAllowedNodeTypeModifications::set_replication_group_id): <p>The name of the replication group want to scale up to a larger node type. ElastiCache uses the replication group id to identify the current node type being used by this replication group, and from that to create a list of node types you can scale up to.</p> <important>   <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p>  </important>
                            /// - On success, responds with [`ListAllowedNodeTypeModificationsOutput`](crate::output::ListAllowedNodeTypeModificationsOutput) with field(s):
    ///   - [`scale_up_modifications(Option<Vec<String>>)`](crate::output::ListAllowedNodeTypeModificationsOutput::scale_up_modifications): <p>A string list, each element of which specifies a cache node type which you can use to scale your cluster or replication group.</p>  <p>When scaling up a Redis cluster or replication group using <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code>, use a value from this list for the <code>CacheNodeType</code> parameter.</p>
    ///   - [`scale_down_modifications(Option<Vec<String>>)`](crate::output::ListAllowedNodeTypeModificationsOutput::scale_down_modifications): <p>A string list, each element of which specifies a cache node type which you can use to scale your cluster or replication group. When scaling down a Redis cluster or replication group using ModifyCacheCluster or ModifyReplicationGroup, use a value from this list for the CacheNodeType parameter. </p>
                            /// - On failure, responds with [`SdkError<ListAllowedNodeTypeModificationsError>`](crate::error::ListAllowedNodeTypeModificationsError)
    pub fn list_allowed_node_type_modifications(&self) -> crate::client::fluent_builders::ListAllowedNodeTypeModifications {
                                crate::client::fluent_builders::ListAllowedNodeTypeModifications::new(self.handle.clone())
                            }
}

