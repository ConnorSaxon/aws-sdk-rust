// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTaskSet`](crate::client::fluent_builders::DeleteTaskSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::client::fluent_builders::DeleteTaskSet::cluster) / [`set_cluster(Option<String>)`](crate::client::fluent_builders::DeleteTaskSet::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set found in to delete.</p>
    ///   - [`service(impl Into<String>)`](crate::client::fluent_builders::DeleteTaskSet::service) / [`set_service(Option<String>)`](crate::client::fluent_builders::DeleteTaskSet::set_service): <p>The short name or full Amazon Resource Name (ARN) of the service that hosts the task set to delete.</p>
    ///   - [`task_set(impl Into<String>)`](crate::client::fluent_builders::DeleteTaskSet::task_set) / [`set_task_set(Option<String>)`](crate::client::fluent_builders::DeleteTaskSet::set_task_set): <p>The task set ID or full Amazon Resource Name (ARN) of the task set to delete.</p>
    ///   - [`force(bool)`](crate::client::fluent_builders::DeleteTaskSet::force) / [`set_force(Option<bool>)`](crate::client::fluent_builders::DeleteTaskSet::set_force): <p>If <code>true</code>, you can delete a task set even if it hasn't been scaled down to zero.</p>
                            /// - On success, responds with [`DeleteTaskSetOutput`](crate::output::DeleteTaskSetOutput) with field(s):
    ///   - [`task_set(Option<TaskSet>)`](crate::output::DeleteTaskSetOutput::task_set): <p>Details about the task set.</p>
                            /// - On failure, responds with [`SdkError<DeleteTaskSetError>`](crate::error::DeleteTaskSetError)
    pub fn delete_task_set(&self) -> crate::client::fluent_builders::DeleteTaskSet {
                                crate::client::fluent_builders::DeleteTaskSet::new(self.handle.clone())
                            }
}

