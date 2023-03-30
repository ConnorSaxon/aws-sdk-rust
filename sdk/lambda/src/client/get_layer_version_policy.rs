// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLayerVersionPolicy`](crate::client::fluent_builders::GetLayerVersionPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`layer_name(impl Into<String>)`](crate::client::fluent_builders::GetLayerVersionPolicy::layer_name) / [`set_layer_name(Option<String>)`](crate::client::fluent_builders::GetLayerVersionPolicy::set_layer_name): <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    ///   - [`version_number(i64)`](crate::client::fluent_builders::GetLayerVersionPolicy::version_number) / [`set_version_number(i64)`](crate::client::fluent_builders::GetLayerVersionPolicy::set_version_number): <p>The version number.</p>
                            /// - On success, responds with [`GetLayerVersionPolicyOutput`](crate::output::GetLayerVersionPolicyOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::output::GetLayerVersionPolicyOutput::policy): <p>The policy document.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::GetLayerVersionPolicyOutput::revision_id): <p>A unique identifier for the current revision of the policy.</p>
                            /// - On failure, responds with [`SdkError<GetLayerVersionPolicyError>`](crate::error::GetLayerVersionPolicyError)
    pub fn get_layer_version_policy(&self) -> crate::client::fluent_builders::GetLayerVersionPolicy {
                                crate::client::fluent_builders::GetLayerVersionPolicy::new(self.handle.clone())
                            }
}

