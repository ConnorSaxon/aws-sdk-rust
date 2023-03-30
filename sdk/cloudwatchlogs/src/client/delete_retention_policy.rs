// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRetentionPolicy`](crate::client::fluent_builders::DeleteRetentionPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`log_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRetentionPolicy::log_group_name) / [`set_log_group_name(Option<String>)`](crate::client::fluent_builders::DeleteRetentionPolicy::set_log_group_name): <p>The name of the log group.</p>
                            /// - On success, responds with [`DeleteRetentionPolicyOutput`](crate::output::DeleteRetentionPolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteRetentionPolicyError>`](crate::error::DeleteRetentionPolicyError)
    pub fn delete_retention_policy(&self) -> crate::client::fluent_builders::DeleteRetentionPolicy {
                                crate::client::fluent_builders::DeleteRetentionPolicy::new(self.handle.clone())
                            }
}

