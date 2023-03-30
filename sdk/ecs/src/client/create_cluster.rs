// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCluster`](crate::client::fluent_builders::CreateCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::client::fluent_builders::CreateCluster::cluster_name) / [`set_cluster_name(Option<String>)`](crate::client::fluent_builders::CreateCluster::set_cluster_name): <p>The name of your cluster. If you don't specify a name for your cluster, you create a cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateCluster::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateCluster::set_tags): <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p>  <p>The following basic restrictions apply to tags:</p>  <ul>   <li> <p>Maximum number of tags per resource - 50</p> </li>   <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>   <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>   <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>   <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>   <li> <p>Tag keys and values are case-sensitive.</p> </li>   <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>  </ul>
    ///   - [`settings(Vec<ClusterSetting>)`](crate::client::fluent_builders::CreateCluster::settings) / [`set_settings(Option<Vec<ClusterSetting>>)`](crate::client::fluent_builders::CreateCluster::set_settings): <p>The setting to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p>
    ///   - [`configuration(ClusterConfiguration)`](crate::client::fluent_builders::CreateCluster::configuration) / [`set_configuration(Option<ClusterConfiguration>)`](crate::client::fluent_builders::CreateCluster::set_configuration): <p>The <code>execute</code> command configuration for the cluster.</p>
    ///   - [`capacity_providers(Vec<String>)`](crate::client::fluent_builders::CreateCluster::capacity_providers) / [`set_capacity_providers(Option<Vec<String>>)`](crate::client::fluent_builders::CreateCluster::set_capacity_providers): <p>The short name of one or more capacity providers to associate with the cluster. A capacity provider must be associated with a cluster before it can be included as part of the default capacity provider strategy of the cluster or used in a capacity provider strategy when calling the <code>CreateService</code> or <code>RunTask</code> actions.</p>  <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must be created but not associated with another cluster. New Auto Scaling group capacity providers can be created with the <code>CreateCapacityProvider</code> API operation.</p>  <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>  <p>The <code>PutClusterCapacityProviders</code> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    ///   - [`default_capacity_provider_strategy(Vec<CapacityProviderStrategyItem>)`](crate::client::fluent_builders::CreateCluster::default_capacity_provider_strategy) / [`set_default_capacity_provider_strategy(Option<Vec<CapacityProviderStrategyItem>>)`](crate::client::fluent_builders::CreateCluster::set_default_capacity_provider_strategy): <p>The capacity provider strategy to set as the default for the cluster. After a default capacity provider strategy is set for a cluster, when you call the <code>RunTask</code> or <code>CreateService</code> APIs with no capacity provider strategy or launch type specified, the default capacity provider strategy for the cluster is used.</p>  <p>If a default capacity provider strategy isn't defined for a cluster when it was created, it can be defined later with the <code>PutClusterCapacityProviders</code> API operation.</p>
    ///   - [`service_connect_defaults(ClusterServiceConnectDefaultsRequest)`](crate::client::fluent_builders::CreateCluster::service_connect_defaults) / [`set_service_connect_defaults(Option<ClusterServiceConnectDefaultsRequest>)`](crate::client::fluent_builders::CreateCluster::set_service_connect_defaults): <p>Use this parameter to set a default Service Connect namespace. After you set a default Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as client services in the namespace. This setting only applies to new services that set the <code>enabled</code> parameter to <code>true</code> in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each service individually in the <code>ServiceConnectConfiguration</code> to override this default parameter.</p>  <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
                            /// - On success, responds with [`CreateClusterOutput`](crate::output::CreateClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::CreateClusterOutput::cluster): <p>The full description of your new cluster.</p>
                            /// - On failure, responds with [`SdkError<CreateClusterError>`](crate::error::CreateClusterError)
    pub fn create_cluster(&self) -> crate::client::fluent_builders::CreateCluster {
                                crate::client::fluent_builders::CreateCluster::new(self.handle.clone())
                            }
}

