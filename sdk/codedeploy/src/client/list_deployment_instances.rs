// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeploymentInstances`](crate::client::fluent_builders::ListDeploymentInstances) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDeploymentInstances::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deployment_id(impl Into<String>)`](crate::client::fluent_builders::ListDeploymentInstances::deployment_id) / [`set_deployment_id(Option<String>)`](crate::client::fluent_builders::ListDeploymentInstances::set_deployment_id): <p> The unique ID of a deployment. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDeploymentInstances::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDeploymentInstances::set_next_token): <p>An identifier returned from the previous list deployment instances call. It can be used to return the next set of deployment instances in the list.</p>
    ///   - [`instance_status_filter(Vec<InstanceStatus>)`](crate::client::fluent_builders::ListDeploymentInstances::instance_status_filter) / [`set_instance_status_filter(Option<Vec<InstanceStatus>>)`](crate::client::fluent_builders::ListDeploymentInstances::set_instance_status_filter): <p>A subset of instances to list by status:</p>  <ul>   <li> <p> <code>Pending</code>: Include those instances with pending deployments.</p> </li>   <li> <p> <code>InProgress</code>: Include those instances where deployments are still in progress.</p> </li>   <li> <p> <code>Succeeded</code>: Include those instances with successful deployments.</p> </li>   <li> <p> <code>Failed</code>: Include those instances with failed deployments.</p> </li>   <li> <p> <code>Skipped</code>: Include those instances with skipped deployments.</p> </li>   <li> <p> <code>Unknown</code>: Include those instances with deployments in an unknown state.</p> </li>  </ul>
    ///   - [`instance_type_filter(Vec<InstanceType>)`](crate::client::fluent_builders::ListDeploymentInstances::instance_type_filter) / [`set_instance_type_filter(Option<Vec<InstanceType>>)`](crate::client::fluent_builders::ListDeploymentInstances::set_instance_type_filter): <p>The set of instances in a blue/green deployment, either those in the original environment ("BLUE") or those in the replacement environment ("GREEN"), for which you want to view instance information.</p>
                            /// - On success, responds with [`ListDeploymentInstancesOutput`](crate::output::ListDeploymentInstancesOutput) with field(s):
    ///   - [`instances_list(Option<Vec<String>>)`](crate::output::ListDeploymentInstancesOutput::instances_list): <p>A list of instance IDs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDeploymentInstancesOutput::next_token): <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment instances call to return the next set of deployment instances in the list.</p>
                            /// - On failure, responds with [`SdkError<ListDeploymentInstancesError>`](crate::error::ListDeploymentInstancesError)
    #[deprecated(note = "This operation is deprecated, use ListDeploymentTargets instead.")]
    pub fn list_deployment_instances(&self) -> crate::client::fluent_builders::ListDeploymentInstances {
                                crate::client::fluent_builders::ListDeploymentInstances::new(self.handle.clone())
                            }
}

