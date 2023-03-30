// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetFirewallRuleGroupPolicy`](crate::client::fluent_builders::GetFirewallRuleGroupPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetFirewallRuleGroupPolicy::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetFirewallRuleGroupPolicy::set_arn): <p>The ARN (Amazon Resource Name) for the rule group.</p>
                            /// - On success, responds with [`GetFirewallRuleGroupPolicyOutput`](crate::output::GetFirewallRuleGroupPolicyOutput) with field(s):
    ///   - [`firewall_rule_group_policy(Option<String>)`](crate::output::GetFirewallRuleGroupPolicyOutput::firewall_rule_group_policy): <p>The Identity and Access Management (Amazon Web Services IAM) policy for sharing the specified rule group. You can use the policy to share the rule group using Resource Access Manager (RAM). </p>
                            /// - On failure, responds with [`SdkError<GetFirewallRuleGroupPolicyError>`](crate::error::GetFirewallRuleGroupPolicyError)
    pub fn get_firewall_rule_group_policy(&self) -> crate::client::fluent_builders::GetFirewallRuleGroupPolicy {
                                crate::client::fluent_builders::GetFirewallRuleGroupPolicy::new(self.handle.clone())
                            }
}

