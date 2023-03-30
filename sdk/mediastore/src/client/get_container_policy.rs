// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetContainerPolicy`](crate::client::fluent_builders::GetContainerPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`container_name(impl Into<String>)`](crate::client::fluent_builders::GetContainerPolicy::container_name) / [`set_container_name(Option<String>)`](crate::client::fluent_builders::GetContainerPolicy::set_container_name): <p>The name of the container. </p>
                            /// - On success, responds with [`GetContainerPolicyOutput`](crate::output::GetContainerPolicyOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::output::GetContainerPolicyOutput::policy): <p>The contents of the access policy.</p>
                            /// - On failure, responds with [`SdkError<GetContainerPolicyError>`](crate::error::GetContainerPolicyError)
    pub fn get_container_policy(&self) -> crate::client::fluent_builders::GetContainerPolicy {
                                crate::client::fluent_builders::GetContainerPolicy::new(self.handle.clone())
                            }
}

