// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SubmitTaskStateChange`](crate::client::fluent_builders::SubmitTaskStateChange) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::cluster) / [`set_cluster(Option<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task.</p>
    ///   - [`task(impl Into<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::task) / [`set_task(Option<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_task): <p>The task ID or full ARN of the task in the state change request.</p>
    ///   - [`status(impl Into<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::status) / [`set_status(Option<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_status): <p>The status of the state change request.</p>
    ///   - [`reason(impl Into<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::reason) / [`set_reason(Option<String>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_reason): <p>The reason for the state change request.</p>
    ///   - [`containers(Vec<ContainerStateChange>)`](crate::client::fluent_builders::SubmitTaskStateChange::containers) / [`set_containers(Option<Vec<ContainerStateChange>>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_containers): <p>Any containers that's associated with the state change request.</p>
    ///   - [`attachments(Vec<AttachmentStateChange>)`](crate::client::fluent_builders::SubmitTaskStateChange::attachments) / [`set_attachments(Option<Vec<AttachmentStateChange>>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_attachments): <p>Any attachments associated with the state change request.</p>
    ///   - [`managed_agents(Vec<ManagedAgentStateChange>)`](crate::client::fluent_builders::SubmitTaskStateChange::managed_agents) / [`set_managed_agents(Option<Vec<ManagedAgentStateChange>>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_managed_agents): <p>The details for the managed agent that's associated with the task.</p>
    ///   - [`pull_started_at(DateTime)`](crate::client::fluent_builders::SubmitTaskStateChange::pull_started_at) / [`set_pull_started_at(Option<DateTime>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_pull_started_at): <p>The Unix timestamp for the time when the container image pull started.</p>
    ///   - [`pull_stopped_at(DateTime)`](crate::client::fluent_builders::SubmitTaskStateChange::pull_stopped_at) / [`set_pull_stopped_at(Option<DateTime>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_pull_stopped_at): <p>The Unix timestamp for the time when the container image pull completed.</p>
    ///   - [`execution_stopped_at(DateTime)`](crate::client::fluent_builders::SubmitTaskStateChange::execution_stopped_at) / [`set_execution_stopped_at(Option<DateTime>)`](crate::client::fluent_builders::SubmitTaskStateChange::set_execution_stopped_at): <p>The Unix timestamp for the time when the task execution stopped.</p>
                            /// - On success, responds with [`SubmitTaskStateChangeOutput`](crate::output::SubmitTaskStateChangeOutput) with field(s):
    ///   - [`acknowledgment(Option<String>)`](crate::output::SubmitTaskStateChangeOutput::acknowledgment): <p>Acknowledgement of the state change.</p>
                            /// - On failure, responds with [`SdkError<SubmitTaskStateChangeError>`](crate::error::SubmitTaskStateChangeError)
    pub fn submit_task_state_change(&self) -> crate::client::fluent_builders::SubmitTaskStateChange {
                                crate::client::fluent_builders::SubmitTaskStateChange::new(self.handle.clone())
                            }
}

