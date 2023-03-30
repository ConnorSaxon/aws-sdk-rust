// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFirewallRules`](crate::client::fluent_builders::ListFirewallRules) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFirewallRules::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`firewall_rule_group_id(impl Into<String>)`](crate::client::fluent_builders::ListFirewallRules::firewall_rule_group_id) / [`set_firewall_rule_group_id(Option<String>)`](crate::client::fluent_builders::ListFirewallRules::set_firewall_rule_group_id): <p>The unique identifier of the firewall rule group that you want to retrieve the rules for. </p>
    ///   - [`priority(i32)`](crate::client::fluent_builders::ListFirewallRules::priority) / [`set_priority(Option<i32>)`](crate::client::fluent_builders::ListFirewallRules::set_priority): <p>Optional additional filter for the rules to retrieve.</p>  <p>The setting that determines the processing order of the rules in a rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
    ///   - [`action(Action)`](crate::client::fluent_builders::ListFirewallRules::action) / [`set_action(Option<Action>)`](crate::client::fluent_builders::ListFirewallRules::set_action): <p>Optional additional filter for the rules to retrieve.</p>  <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:</p>  <ul>   <li> <p> <code>ALLOW</code> - Permit the request to go through.</p> </li>   <li> <p> <code>ALERT</code> - Permit the request to go through but send an alert to the logs.</p> </li>   <li> <p> <code>BLOCK</code> - Disallow the request. If this is specified, additional handling details are provided in the rule's <code>BlockResponse</code> setting. </p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFirewallRules::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFirewallRules::set_max_results): <p>The maximum number of objects that you want Resolver to return for this request. If more objects are available, in the response, Resolver provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p>  <p>If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 objects. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFirewallRules::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFirewallRules::set_next_token): <p>For the first call to this list request, omit this value.</p>  <p>When you request a list of objects, Resolver returns at most the number of objects specified in <code>MaxResults</code>. If more objects are available for retrieval, Resolver returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token that was returned for the prior request in your next request.</p>
                            /// - On success, responds with [`ListFirewallRulesOutput`](crate::output::ListFirewallRulesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListFirewallRulesOutput::next_token): <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    ///   - [`firewall_rules(Option<Vec<FirewallRule>>)`](crate::output::ListFirewallRulesOutput::firewall_rules): <p>A list of the rules that you have defined. </p>  <p>This might be a partial list of the firewall rules that you've defined. For information, see <code>MaxResults</code>. </p>
                            /// - On failure, responds with [`SdkError<ListFirewallRulesError>`](crate::error::ListFirewallRulesError)
    pub fn list_firewall_rules(&self) -> crate::client::fluent_builders::ListFirewallRules {
                                crate::client::fluent_builders::ListFirewallRules::new(self.handle.clone())
                            }
}

