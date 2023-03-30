// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeploymentGroups`](crate::client::fluent_builders::ListDeploymentGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDeploymentGroups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::ListDeploymentGroups::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::ListDeploymentGroups::set_application_name): <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDeploymentGroups::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDeploymentGroups::set_next_token): <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
                            /// - On success, responds with [`ListDeploymentGroupsOutput`](crate::output::ListDeploymentGroupsOutput) with field(s):
    ///   - [`application_name(Option<String>)`](crate::output::ListDeploymentGroupsOutput::application_name): <p>The application name.</p>
    ///   - [`deployment_groups(Option<Vec<String>>)`](crate::output::ListDeploymentGroupsOutput::deployment_groups): <p>A list of deployment group names.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDeploymentGroupsOutput::next_token): <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment groups call to return the next set of deployment groups in the list.</p>
                            /// - On failure, responds with [`SdkError<ListDeploymentGroupsError>`](crate::error::ListDeploymentGroupsError)
    pub fn list_deployment_groups(&self) -> crate::client::fluent_builders::ListDeploymentGroups {
                                crate::client::fluent_builders::ListDeploymentGroups::new(self.handle.clone())
                            }
}

