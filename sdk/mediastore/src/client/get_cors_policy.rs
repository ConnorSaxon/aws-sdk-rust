// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCorsPolicy`](crate::client::fluent_builders::GetCorsPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`container_name(impl Into<String>)`](crate::client::fluent_builders::GetCorsPolicy::container_name) / [`set_container_name(Option<String>)`](crate::client::fluent_builders::GetCorsPolicy::set_container_name): <p>The name of the container that the policy is assigned to.</p>
                            /// - On success, responds with [`GetCorsPolicyOutput`](crate::output::GetCorsPolicyOutput) with field(s):
    ///   - [`cors_policy(Option<Vec<CorsRule>>)`](crate::output::GetCorsPolicyOutput::cors_policy): <p>The CORS policy assigned to the container.</p>
                            /// - On failure, responds with [`SdkError<GetCorsPolicyError>`](crate::error::GetCorsPolicyError)
    pub fn get_cors_policy(&self) -> crate::client::fluent_builders::GetCorsPolicy {
                                crate::client::fluent_builders::GetCorsPolicy::new(self.handle.clone())
                            }
}

