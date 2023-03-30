// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetApplicationPolicy`](crate::client::fluent_builders::GetApplicationPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetApplicationPolicy::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetApplicationPolicy::set_application_id): <p>The Amazon Resource Name (ARN) of the application.</p>
                            /// - On success, responds with [`GetApplicationPolicyOutput`](crate::output::GetApplicationPolicyOutput) with field(s):
    ///   - [`statements(Option<Vec<ApplicationPolicyStatement>>)`](crate::output::GetApplicationPolicyOutput::statements): <p>An array of policy statements applied to the application.</p>
                            /// - On failure, responds with [`SdkError<GetApplicationPolicyError>`](crate::error::GetApplicationPolicyError)
    pub fn get_application_policy(&self) -> crate::client::fluent_builders::GetApplicationPolicy {
                                crate::client::fluent_builders::GetApplicationPolicy::new(self.handle.clone())
                            }
}

