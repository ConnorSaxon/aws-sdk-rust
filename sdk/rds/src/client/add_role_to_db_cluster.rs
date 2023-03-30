// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddRoleToDBCluster`](crate::client::fluent_builders::AddRoleToDBCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::set_db_cluster_identifier): <p>The name of the DB cluster to associate the IAM role with.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::set_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role to associate with the Aurora DB cluster, for example <code>arn:aws:iam::123456789012:role/AuroraAccessRole</code>.</p>
    ///   - [`feature_name(impl Into<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::feature_name) / [`set_feature_name(Option<String>)`](crate::client::fluent_builders::AddRoleToDBCluster::set_feature_name): <p>The name of the feature for the DB cluster that the IAM role is to be associated with. For information about supported feature names, see <code>DBEngineVersion</code>.</p>
                            /// - On success, responds with [`AddRoleToDbClusterOutput`](crate::output::AddRoleToDbClusterOutput)
                            /// - On failure, responds with [`SdkError<AddRoleToDBClusterError>`](crate::error::AddRoleToDBClusterError)
    pub fn add_role_to_db_cluster(&self) -> crate::client::fluent_builders::AddRoleToDBCluster {
                                crate::client::fluent_builders::AddRoleToDBCluster::new(self.handle.clone())
                            }
}

