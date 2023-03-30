// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRepositoryPolicy`](crate::client::fluent_builders::DeleteRepositoryPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`registry_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRepositoryPolicy::registry_id) / [`set_registry_id(Option<String>)`](crate::client::fluent_builders::DeleteRepositoryPolicy::set_registry_id): <p>The Amazon Web Services account ID that's associated with the public registry that contains the repository policy to delete. If you do not specify a registry, the default public registry is assumed.</p>
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRepositoryPolicy::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::DeleteRepositoryPolicy::set_repository_name): <p>The name of the repository that's associated with the repository policy to delete.</p>
                            /// - On success, responds with [`DeleteRepositoryPolicyOutput`](crate::output::DeleteRepositoryPolicyOutput) with field(s):
    ///   - [`registry_id(Option<String>)`](crate::output::DeleteRepositoryPolicyOutput::registry_id): <p>The registry ID that's associated with the request.</p>
    ///   - [`repository_name(Option<String>)`](crate::output::DeleteRepositoryPolicyOutput::repository_name): <p>The repository name that's associated with the request.</p>
    ///   - [`policy_text(Option<String>)`](crate::output::DeleteRepositoryPolicyOutput::policy_text): <p>The JSON repository policy that was deleted from the repository.</p>
                            /// - On failure, responds with [`SdkError<DeleteRepositoryPolicyError>`](crate::error::DeleteRepositoryPolicyError)
    pub fn delete_repository_policy(&self) -> crate::client::fluent_builders::DeleteRepositoryPolicy {
                                crate::client::fluent_builders::DeleteRepositoryPolicy::new(self.handle.clone())
                            }
}

