// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRepository`](crate::client::fluent_builders::DeleteRepository) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRepository::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::DeleteRepository::set_repository_name): <p>The name of the repository to delete.</p>
                            /// - On success, responds with [`DeleteRepositoryOutput`](crate::output::DeleteRepositoryOutput) with field(s):
    ///   - [`repository_id(Option<String>)`](crate::output::DeleteRepositoryOutput::repository_id): <p>The ID of the repository that was deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteRepositoryError>`](crate::error::DeleteRepositoryError)
    pub fn delete_repository(&self) -> crate::client::fluent_builders::DeleteRepository {
                                crate::client::fluent_builders::DeleteRepository::new(self.handle.clone())
                            }
}

