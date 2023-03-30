// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateContinuousDeploymentPolicy`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`continuous_deployment_policy_config(ContinuousDeploymentPolicyConfig)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::continuous_deployment_policy_config) / [`set_continuous_deployment_policy_config(Option<ContinuousDeploymentPolicyConfig>)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::set_continuous_deployment_policy_config): <p>The continuous deployment policy configuration.</p>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::set_id): <p>The identifier of the continuous deployment policy that you are updating.</p>
    ///   - [`if_match(impl Into<String>)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::if_match) / [`set_if_match(Option<String>)`](crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::set_if_match): <p>The current version (<code>ETag</code> value) of the continuous deployment policy that you are updating.</p>
                            /// - On success, responds with [`UpdateContinuousDeploymentPolicyOutput`](crate::output::UpdateContinuousDeploymentPolicyOutput) with field(s):
    ///   - [`continuous_deployment_policy(Option<ContinuousDeploymentPolicy>)`](crate::output::UpdateContinuousDeploymentPolicyOutput::continuous_deployment_policy): <p>A continuous deployment policy.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::UpdateContinuousDeploymentPolicyOutput::e_tag): <p>The version identifier for the current version of the continuous deployment policy.</p>
                            /// - On failure, responds with [`SdkError<UpdateContinuousDeploymentPolicyError>`](crate::error::UpdateContinuousDeploymentPolicyError)
    pub fn update_continuous_deployment_policy(&self) -> crate::client::fluent_builders::UpdateContinuousDeploymentPolicy {
                                crate::client::fluent_builders::UpdateContinuousDeploymentPolicy::new(self.handle.clone())
                            }
}

