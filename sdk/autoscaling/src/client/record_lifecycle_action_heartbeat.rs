// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RecordLifecycleActionHeartbeat`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`lifecycle_hook_name(impl Into<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::lifecycle_hook_name) / [`set_lifecycle_hook_name(Option<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::set_lifecycle_hook_name): <p>The name of the lifecycle hook.</p>
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`lifecycle_action_token(impl Into<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::lifecycle_action_token) / [`set_lifecycle_action_token(Option<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::set_lifecycle_action_token): <p>A token that uniquely identifies a specific lifecycle action associated with an instance. Amazon EC2 Auto Scaling sends this token to the notification target that you specified when you created the lifecycle hook.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::RecordLifecycleActionHeartbeat::set_instance_id): <p>The ID of the instance.</p>
                            /// - On success, responds with [`RecordLifecycleActionHeartbeatOutput`](crate::output::RecordLifecycleActionHeartbeatOutput)
                            /// - On failure, responds with [`SdkError<RecordLifecycleActionHeartbeatError>`](crate::error::RecordLifecycleActionHeartbeatError)
    pub fn record_lifecycle_action_heartbeat(&self) -> crate::client::fluent_builders::RecordLifecycleActionHeartbeat {
                                crate::client::fluent_builders::RecordLifecycleActionHeartbeat::new(self.handle.clone())
                            }
}

