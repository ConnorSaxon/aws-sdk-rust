// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRuleGroupsNamespace`](crate::client::fluent_builders::CreateRuleGroupsNamespace) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::set_workspace_id): The ID of the workspace in which to create the rule group namespace.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::set_name): The rule groups namespace name.
    ///   - [`data(Blob)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::data) / [`set_data(Option<Blob>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::set_data): The namespace data that define the rule groups.
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::set_client_token): Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateRuleGroupsNamespace::set_tags): Optional, user-provided tags for this rule groups namespace.
                            /// - On success, responds with [`CreateRuleGroupsNamespaceOutput`](crate::output::CreateRuleGroupsNamespaceOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateRuleGroupsNamespaceOutput::name): The rule groups namespace name.
    ///   - [`arn(Option<String>)`](crate::output::CreateRuleGroupsNamespaceOutput::arn): The Amazon Resource Name (ARN) of this rule groups namespace.
    ///   - [`status(Option<RuleGroupsNamespaceStatus>)`](crate::output::CreateRuleGroupsNamespaceOutput::status): The status of rule groups namespace.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateRuleGroupsNamespaceOutput::tags): The tags of this rule groups namespace.
                            /// - On failure, responds with [`SdkError<CreateRuleGroupsNamespaceError>`](crate::error::CreateRuleGroupsNamespaceError)
    pub fn create_rule_groups_namespace(&self) -> crate::client::fluent_builders::CreateRuleGroupsNamespace {
                                crate::client::fluent_builders::CreateRuleGroupsNamespace::new(self.handle.clone())
                            }
}

