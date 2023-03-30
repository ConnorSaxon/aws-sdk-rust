// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAutoScalingGroup`](crate::client::fluent_builders::DeleteAutoScalingGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteAutoScalingGroup::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::DeleteAutoScalingGroup::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`force_delete(bool)`](crate::client::fluent_builders::DeleteAutoScalingGroup::force_delete) / [`set_force_delete(Option<bool>)`](crate::client::fluent_builders::DeleteAutoScalingGroup::set_force_delete): <p>Specifies that the group is to be deleted along with all instances associated with the group, without waiting for all instances to be terminated. This action also deletes any outstanding lifecycle actions associated with the group.</p>
                            /// - On success, responds with [`DeleteAutoScalingGroupOutput`](crate::output::DeleteAutoScalingGroupOutput)
                            /// - On failure, responds with [`SdkError<DeleteAutoScalingGroupError>`](crate::error::DeleteAutoScalingGroupError)
    pub fn delete_auto_scaling_group(&self) -> crate::client::fluent_builders::DeleteAutoScalingGroup {
                                crate::client::fluent_builders::DeleteAutoScalingGroup::new(self.handle.clone())
                            }
}

