// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateApprovalRuleTemplateContent`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`approval_rule_template_name(impl Into<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::approval_rule_template_name) / [`set_approval_rule_template_name(Option<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::set_approval_rule_template_name): <p>The name of the approval rule template where you want to update the content of the rule. </p>
    ///   - [`new_rule_content(impl Into<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::new_rule_content) / [`set_new_rule_content(Option<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::set_new_rule_content): <p>The content that replaces the existing content of the rule. Content statements must be complete. You cannot provide only the changes.</p>
    ///   - [`existing_rule_content_sha256(impl Into<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::existing_rule_content_sha256) / [`set_existing_rule_content_sha256(Option<String>)`](crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::set_existing_rule_content_sha256): <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this information by using <code>GetPullRequest</code>.</p>
                            /// - On success, responds with [`UpdateApprovalRuleTemplateContentOutput`](crate::output::UpdateApprovalRuleTemplateContentOutput) with field(s):
    ///   - [`approval_rule_template(Option<ApprovalRuleTemplate>)`](crate::output::UpdateApprovalRuleTemplateContentOutput::approval_rule_template): <p>Returns information about an approval rule template.</p>
                            /// - On failure, responds with [`SdkError<UpdateApprovalRuleTemplateContentError>`](crate::error::UpdateApprovalRuleTemplateContentError)
    pub fn update_approval_rule_template_content(&self) -> crate::client::fluent_builders::UpdateApprovalRuleTemplateContent {
                                crate::client::fluent_builders::UpdateApprovalRuleTemplateContent::new(self.handle.clone())
                            }
}

