// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateClusterV2`](crate::client::fluent_builders::CreateClusterV2) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::client::fluent_builders::CreateClusterV2::cluster_name) / [`set_cluster_name(Option<String>)`](crate::client::fluent_builders::CreateClusterV2::set_cluster_name): <p>The name of the cluster.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateClusterV2::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateClusterV2::set_tags): <p>A map of tags that you want the cluster to have.</p>
    ///   - [`provisioned(ProvisionedRequest)`](crate::client::fluent_builders::CreateClusterV2::provisioned) / [`set_provisioned(Option<ProvisionedRequest>)`](crate::client::fluent_builders::CreateClusterV2::set_provisioned): <p>Information about the provisioned cluster.</p>
    ///   - [`serverless(ServerlessRequest)`](crate::client::fluent_builders::CreateClusterV2::serverless) / [`set_serverless(Option<ServerlessRequest>)`](crate::client::fluent_builders::CreateClusterV2::set_serverless): <p>Information about the serverless cluster.</p>
                            /// - On success, responds with [`CreateClusterV2Output`](crate::output::CreateClusterV2Output) with field(s):
    ///   - [`cluster_arn(Option<String>)`](crate::output::CreateClusterV2Output::cluster_arn): <p>The Amazon Resource Name (ARN) of the cluster.</p>
    ///   - [`cluster_name(Option<String>)`](crate::output::CreateClusterV2Output::cluster_name): <p>The name of the MSK cluster.</p>
    ///   - [`state(Option<ClusterState>)`](crate::output::CreateClusterV2Output::state): <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    ///   - [`cluster_type(Option<ClusterType>)`](crate::output::CreateClusterV2Output::cluster_type): <p>The type of the cluster. The possible states are PROVISIONED or SERVERLESS.</p>
                            /// - On failure, responds with [`SdkError<CreateClusterV2Error>`](crate::error::CreateClusterV2Error)
    pub fn create_cluster_v2(&self) -> crate::client::fluent_builders::CreateClusterV2 {
                                crate::client::fluent_builders::CreateClusterV2::new(self.handle.clone())
                            }
}

