// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutLifecyclePolicy`](crate::client::fluent_builders::PutLifecyclePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`container_name(impl Into<String>)`](crate::client::fluent_builders::PutLifecyclePolicy::container_name) / [`set_container_name(Option<String>)`](crate::client::fluent_builders::PutLifecyclePolicy::set_container_name): <p>The name of the container that you want to assign the object lifecycle policy to.</p>
    ///   - [`lifecycle_policy(impl Into<String>)`](crate::client::fluent_builders::PutLifecyclePolicy::lifecycle_policy) / [`set_lifecycle_policy(Option<String>)`](crate::client::fluent_builders::PutLifecyclePolicy::set_lifecycle_policy): <p>The object lifecycle policy to apply to the container.</p>
                            /// - On success, responds with [`PutLifecyclePolicyOutput`](crate::output::PutLifecyclePolicyOutput)
                            /// - On failure, responds with [`SdkError<PutLifecyclePolicyError>`](crate::error::PutLifecyclePolicyError)
    pub fn put_lifecycle_policy(&self) -> crate::client::fluent_builders::PutLifecyclePolicy {
                                crate::client::fluent_builders::PutLifecyclePolicy::new(self.handle.clone())
                            }
}

