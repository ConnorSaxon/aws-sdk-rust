// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateServicePrimaryTaskSet`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::cluster) / [`set_cluster(Option<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set exists in.</p>
    ///   - [`service(impl Into<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::service) / [`set_service(Option<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::set_service): <p>The short name or full Amazon Resource Name (ARN) of the service that the task set exists in.</p>
    ///   - [`primary_task_set(impl Into<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::primary_task_set) / [`set_primary_task_set(Option<String>)`](crate::client::fluent_builders::UpdateServicePrimaryTaskSet::set_primary_task_set): <p>The short name or full Amazon Resource Name (ARN) of the task set to set as the primary task set in the deployment.</p>
                            /// - On success, responds with [`UpdateServicePrimaryTaskSetOutput`](crate::output::UpdateServicePrimaryTaskSetOutput) with field(s):
    ///   - [`task_set(Option<TaskSet>)`](crate::output::UpdateServicePrimaryTaskSetOutput::task_set): <p>The details about the task set.</p>
                            /// - On failure, responds with [`SdkError<UpdateServicePrimaryTaskSetError>`](crate::error::UpdateServicePrimaryTaskSetError)
    pub fn update_service_primary_task_set(&self) -> crate::client::fluent_builders::UpdateServicePrimaryTaskSet {
                                crate::client::fluent_builders::UpdateServicePrimaryTaskSet::new(self.handle.clone())
                            }
}

