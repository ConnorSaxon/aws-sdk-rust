// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApprovalRuleTemplate`](crate::client::fluent_builders::CreateApprovalRuleTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`approval_rule_template_name(impl Into<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::approval_rule_template_name) / [`set_approval_rule_template_name(Option<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::set_approval_rule_template_name): <p>The name of the approval rule template. Provide descriptive names, because this name is applied to the approval rules created automatically in associated repositories.</p>
    ///   - [`approval_rule_template_content(impl Into<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::approval_rule_template_content) / [`set_approval_rule_template_content(Option<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::set_approval_rule_template_content): <p>The content of the approval rule that is created on pull requests in associated repositories. If you specify one or more destination references (branches), approval rules are created in an associated repository only if their destination references (branches) match those specified in the template.</p> <note>   <p>When you create the content of the approval rule template, you can specify approvers in an approval pool in one of two ways:</p>   <ul>    <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary_Major</i>, all of the following are counted as approvals coming from that user:</p>     <ul>      <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p> </li>      <li> <p>A federated user identified in IAM as Mary_Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p> </li>     </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary_Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>) unless you include a wildcard (*Mary_Major).</p> </li>    <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li>   </ul>   <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>  </note>
    ///   - [`approval_rule_template_description(impl Into<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::approval_rule_template_description) / [`set_approval_rule_template_description(Option<String>)`](crate::client::fluent_builders::CreateApprovalRuleTemplate::set_approval_rule_template_description): <p>The description of the approval rule template. Consider providing a description that explains what this template does and when it might be appropriate to associate it with repositories.</p>
                            /// - On success, responds with [`CreateApprovalRuleTemplateOutput`](crate::output::CreateApprovalRuleTemplateOutput) with field(s):
    ///   - [`approval_rule_template(Option<ApprovalRuleTemplate>)`](crate::output::CreateApprovalRuleTemplateOutput::approval_rule_template): <p>The content and structure of the created approval rule template.</p>
                            /// - On failure, responds with [`SdkError<CreateApprovalRuleTemplateError>`](crate::error::CreateApprovalRuleTemplateError)
    pub fn create_approval_rule_template(&self) -> crate::client::fluent_builders::CreateApprovalRuleTemplate {
                                crate::client::fluent_builders::CreateApprovalRuleTemplate::new(self.handle.clone())
                            }
}

