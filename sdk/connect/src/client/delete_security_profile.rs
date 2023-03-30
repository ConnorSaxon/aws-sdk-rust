// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSecurityProfile`](crate::client::fluent_builders::DeleteSecurityProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DeleteSecurityProfile::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DeleteSecurityProfile::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`security_profile_id(impl Into<String>)`](crate::client::fluent_builders::DeleteSecurityProfile::security_profile_id) / [`set_security_profile_id(Option<String>)`](crate::client::fluent_builders::DeleteSecurityProfile::set_security_profile_id): <p>The identifier for the security profle.</p>
                            /// - On success, responds with [`DeleteSecurityProfileOutput`](crate::output::DeleteSecurityProfileOutput)
                            /// - On failure, responds with [`SdkError<DeleteSecurityProfileError>`](crate::error::DeleteSecurityProfileError)
    pub fn delete_security_profile(&self) -> crate::client::fluent_builders::DeleteSecurityProfile {
                                crate::client::fluent_builders::DeleteSecurityProfile::new(self.handle.clone())
                            }
}

