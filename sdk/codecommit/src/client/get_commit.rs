// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCommit`](crate::client::fluent_builders::GetCommit) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::GetCommit::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::GetCommit::set_repository_name): <p>The name of the repository to which the commit was made.</p>
    ///   - [`commit_id(impl Into<String>)`](crate::client::fluent_builders::GetCommit::commit_id) / [`set_commit_id(Option<String>)`](crate::client::fluent_builders::GetCommit::set_commit_id): <p>The commit ID. Commit IDs are the full SHA ID of the commit.</p>
                            /// - On success, responds with [`GetCommitOutput`](crate::output::GetCommitOutput) with field(s):
    ///   - [`commit(Option<Commit>)`](crate::output::GetCommitOutput::commit): <p>A commit data type object that contains information about the specified commit.</p>
                            /// - On failure, responds with [`SdkError<GetCommitError>`](crate::error::GetCommitError)
    pub fn get_commit(&self) -> crate::client::fluent_builders::GetCommit {
                                crate::client::fluent_builders::GetCommit::new(self.handle.clone())
                            }
}

