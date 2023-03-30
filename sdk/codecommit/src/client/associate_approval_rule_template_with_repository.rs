// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateApprovalRuleTemplateWithRepository`](crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`approval_rule_template_name(impl Into<String>)`](crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository::approval_rule_template_name) / [`set_approval_rule_template_name(Option<String>)`](crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository::set_approval_rule_template_name): <p>The name for the approval rule template. </p>
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository::set_repository_name): <p>The name of the repository that you want to associate with the template.</p>
                            /// - On success, responds with [`AssociateApprovalRuleTemplateWithRepositoryOutput`](crate::output::AssociateApprovalRuleTemplateWithRepositoryOutput)
                            /// - On failure, responds with [`SdkError<AssociateApprovalRuleTemplateWithRepositoryError>`](crate::error::AssociateApprovalRuleTemplateWithRepositoryError)
    pub fn associate_approval_rule_template_with_repository(&self) -> crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository {
                                crate::client::fluent_builders::AssociateApprovalRuleTemplateWithRepository::new(self.handle.clone())
                            }
}

