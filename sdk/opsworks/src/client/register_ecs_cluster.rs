// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RegisterEcsCluster`](crate::client::fluent_builders::RegisterEcsCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`ecs_cluster_arn(impl Into<String>)`](crate::client::fluent_builders::RegisterEcsCluster::ecs_cluster_arn) / [`set_ecs_cluster_arn(Option<String>)`](crate::client::fluent_builders::RegisterEcsCluster::set_ecs_cluster_arn): <p>The cluster's ARN.</p>
    ///   - [`stack_id(impl Into<String>)`](crate::client::fluent_builders::RegisterEcsCluster::stack_id) / [`set_stack_id(Option<String>)`](crate::client::fluent_builders::RegisterEcsCluster::set_stack_id): <p>The stack ID.</p>
                            /// - On success, responds with [`RegisterEcsClusterOutput`](crate::output::RegisterEcsClusterOutput) with field(s):
    ///   - [`ecs_cluster_arn(Option<String>)`](crate::output::RegisterEcsClusterOutput::ecs_cluster_arn): <p>The cluster's ARN.</p>
                            /// - On failure, responds with [`SdkError<RegisterEcsClusterError>`](crate::error::RegisterEcsClusterError)
    pub fn register_ecs_cluster(&self) -> crate::client::fluent_builders::RegisterEcsCluster {
                                crate::client::fluent_builders::RegisterEcsCluster::new(self.handle.clone())
                            }
}

