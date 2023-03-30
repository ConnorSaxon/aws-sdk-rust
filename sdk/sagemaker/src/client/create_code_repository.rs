// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCodeRepository`](crate::client::fluent_builders::CreateCodeRepository) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`code_repository_name(impl Into<String>)`](crate::client::fluent_builders::CreateCodeRepository::code_repository_name) / [`set_code_repository_name(Option<String>)`](crate::client::fluent_builders::CreateCodeRepository::set_code_repository_name): <p>The name of the Git repository. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    ///   - [`git_config(GitConfig)`](crate::client::fluent_builders::CreateCodeRepository::git_config) / [`set_git_config(Option<GitConfig>)`](crate::client::fluent_builders::CreateCodeRepository::set_git_config): <p>Specifies details about the repository, including the URL where the repository is located, the default branch, and credentials to use to access the repository.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateCodeRepository::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateCodeRepository::set_tags): <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
                            /// - On success, responds with [`CreateCodeRepositoryOutput`](crate::output::CreateCodeRepositoryOutput) with field(s):
    ///   - [`code_repository_arn(Option<String>)`](crate::output::CreateCodeRepositoryOutput::code_repository_arn): <p>The Amazon Resource Name (ARN) of the new repository.</p>
                            /// - On failure, responds with [`SdkError<CreateCodeRepositoryError>`](crate::error::CreateCodeRepositoryError)
    pub fn create_code_repository(&self) -> crate::client::fluent_builders::CreateCodeRepository {
                                crate::client::fluent_builders::CreateCodeRepository::new(self.handle.clone())
                            }
}

