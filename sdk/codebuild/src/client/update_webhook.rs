// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWebhook`](crate::client::fluent_builders::UpdateWebhook) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_name(impl Into<String>)`](crate::client::fluent_builders::UpdateWebhook::project_name) / [`set_project_name(Option<String>)`](crate::client::fluent_builders::UpdateWebhook::set_project_name): <p>The name of the CodeBuild project.</p>
    ///   - [`branch_filter(impl Into<String>)`](crate::client::fluent_builders::UpdateWebhook::branch_filter) / [`set_branch_filter(Option<String>)`](crate::client::fluent_builders::UpdateWebhook::set_branch_filter): <p>A regular expression used to determine which repository branches are built when a webhook is triggered. If the name of a branch matches the regular expression, then it is built. If <code>branchFilter</code> is empty, then all branches are built.</p> <note>   <p> It is recommended that you use <code>filterGroups</code> instead of <code>branchFilter</code>. </p>  </note>
    ///   - [`rotate_secret(bool)`](crate::client::fluent_builders::UpdateWebhook::rotate_secret) / [`set_rotate_secret(bool)`](crate::client::fluent_builders::UpdateWebhook::set_rotate_secret): <p> A boolean value that specifies whether the associated GitHub repository's secret token should be updated. If you use Bitbucket for your repository, <code>rotateSecret</code> is ignored. </p>
    ///   - [`filter_groups(Vec<Vec<WebhookFilter>>)`](crate::client::fluent_builders::UpdateWebhook::filter_groups) / [`set_filter_groups(Option<Vec<Vec<WebhookFilter>>>)`](crate::client::fluent_builders::UpdateWebhook::set_filter_groups): <p> An array of arrays of <code>WebhookFilter</code> objects used to determine if a webhook event can trigger a build. A filter group must contain at least one <code>EVENT</code> <code>WebhookFilter</code>. </p>
    ///   - [`build_type(WebhookBuildType)`](crate::client::fluent_builders::UpdateWebhook::build_type) / [`set_build_type(Option<WebhookBuildType>)`](crate::client::fluent_builders::UpdateWebhook::set_build_type): <p>Specifies the type of build this webhook will trigger.</p>
                            /// - On success, responds with [`UpdateWebhookOutput`](crate::output::UpdateWebhookOutput) with field(s):
    ///   - [`webhook(Option<Webhook>)`](crate::output::UpdateWebhookOutput::webhook): <p> Information about a repository's webhook that is associated with a project in CodeBuild. </p>
                            /// - On failure, responds with [`SdkError<UpdateWebhookError>`](crate::error::UpdateWebhookError)
    pub fn update_webhook(&self) -> crate::client::fluent_builders::UpdateWebhook {
                                crate::client::fluent_builders::UpdateWebhook::new(self.handle.clone())
                            }
}

