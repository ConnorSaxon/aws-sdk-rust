// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListActivatedRulesInRuleGroup`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rule_group_id(impl Into<String>)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::rule_group_id) / [`set_rule_group_id(Option<String>)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::set_rule_group_id): <p>The <code>RuleGroupId</code> of the <code>RuleGroup</code> for which you want to get a list of <code>ActivatedRule</code> objects.</p>
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::set_next_marker): <p>If you specify a value for <code>Limit</code> and you have more <code>ActivatedRules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ActivatedRules</code>. For the second and subsequent <code>ListActivatedRulesInRuleGroup</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ActivatedRules</code>.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::limit) / [`set_limit(i32)`](crate::client::fluent_builders::ListActivatedRulesInRuleGroup::set_limit): <p>Specifies the number of <code>ActivatedRules</code> that you want AWS WAF to return for this request. If you have more <code>ActivatedRules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ActivatedRules</code>.</p>
                            /// - On success, responds with [`ListActivatedRulesInRuleGroupOutput`](crate::output::ListActivatedRulesInRuleGroupOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListActivatedRulesInRuleGroupOutput::next_marker): <p>If you have more <code>ActivatedRules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ActivatedRules</code>, submit another <code>ListActivatedRulesInRuleGroup</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    ///   - [`activated_rules(Option<Vec<ActivatedRule>>)`](crate::output::ListActivatedRulesInRuleGroupOutput::activated_rules): <p>An array of <code>ActivatedRules</code> objects.</p>
                            /// - On failure, responds with [`SdkError<ListActivatedRulesInRuleGroupError>`](crate::error::ListActivatedRulesInRuleGroupError)
    pub fn list_activated_rules_in_rule_group(&self) -> crate::client::fluent_builders::ListActivatedRulesInRuleGroup {
                                crate::client::fluent_builders::ListActivatedRulesInRuleGroup::new(self.handle.clone())
                            }
}

