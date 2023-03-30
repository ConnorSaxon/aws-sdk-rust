// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePullRequestDescription`](crate::client::fluent_builders::UpdatePullRequestDescription) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pull_request_id(impl Into<String>)`](crate::client::fluent_builders::UpdatePullRequestDescription::pull_request_id) / [`set_pull_request_id(Option<String>)`](crate::client::fluent_builders::UpdatePullRequestDescription::set_pull_request_id): <p>The system-generated ID of the pull request. To get this ID, use <code>ListPullRequests</code>.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdatePullRequestDescription::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdatePullRequestDescription::set_description): <p>The updated content of the description for the pull request. This content replaces the existing description.</p>
                            /// - On success, responds with [`UpdatePullRequestDescriptionOutput`](crate::output::UpdatePullRequestDescriptionOutput) with field(s):
    ///   - [`pull_request(Option<PullRequest>)`](crate::output::UpdatePullRequestDescriptionOutput::pull_request): <p>Information about the updated pull request.</p>
                            /// - On failure, responds with [`SdkError<UpdatePullRequestDescriptionError>`](crate::error::UpdatePullRequestDescriptionError)
    pub fn update_pull_request_description(&self) -> crate::client::fluent_builders::UpdatePullRequestDescription {
                                crate::client::fluent_builders::UpdatePullRequestDescription::new(self.handle.clone())
                            }
}

