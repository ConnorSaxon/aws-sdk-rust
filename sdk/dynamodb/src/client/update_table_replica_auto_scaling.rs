// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTableReplicaAutoScaling`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_secondary_index_updates(Vec<GlobalSecondaryIndexAutoScalingUpdate>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::global_secondary_index_updates) / [`set_global_secondary_index_updates(Option<Vec<GlobalSecondaryIndexAutoScalingUpdate>>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::set_global_secondary_index_updates): <p>Represents the auto scaling settings of the global secondary indexes of the replica to be updated.</p>
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::set_table_name): <p>The name of the global table to be updated.</p>
    ///   - [`provisioned_write_capacity_auto_scaling_update(AutoScalingSettingsUpdate)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::provisioned_write_capacity_auto_scaling_update) / [`set_provisioned_write_capacity_auto_scaling_update(Option<AutoScalingSettingsUpdate>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::set_provisioned_write_capacity_auto_scaling_update): <p>Represents the auto scaling settings to be modified for a global table or global secondary index.</p>
    ///   - [`replica_updates(Vec<ReplicaAutoScalingUpdate>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::replica_updates) / [`set_replica_updates(Option<Vec<ReplicaAutoScalingUpdate>>)`](crate::client::fluent_builders::UpdateTableReplicaAutoScaling::set_replica_updates): <p>Represents the auto scaling settings of replicas of the table that will be modified.</p>
                            /// - On success, responds with [`UpdateTableReplicaAutoScalingOutput`](crate::output::UpdateTableReplicaAutoScalingOutput) with field(s):
    ///   - [`table_auto_scaling_description(Option<TableAutoScalingDescription>)`](crate::output::UpdateTableReplicaAutoScalingOutput::table_auto_scaling_description): <p>Returns information about the auto scaling settings of a table with replicas.</p>
                            /// - On failure, responds with [`SdkError<UpdateTableReplicaAutoScalingError>`](crate::error::UpdateTableReplicaAutoScalingError)
    pub fn update_table_replica_auto_scaling(&self) -> crate::client::fluent_builders::UpdateTableReplicaAutoScaling {
                                crate::client::fluent_builders::UpdateTableReplicaAutoScaling::new(self.handle.clone())
                            }
}

