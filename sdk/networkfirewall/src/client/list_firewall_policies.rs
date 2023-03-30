// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFirewallPolicies`](crate::client::fluent_builders::ListFirewallPolicies) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFirewallPolicies::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFirewallPolicies::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFirewallPolicies::set_next_token): <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFirewallPolicies::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFirewallPolicies::set_max_results): <p>The maximum number of objects that you want Network Firewall to return for this request. If more objects are available, in the response, Network Firewall provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p>
                            /// - On success, responds with [`ListFirewallPoliciesOutput`](crate::output::ListFirewallPoliciesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListFirewallPoliciesOutput::next_token): <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p>
    ///   - [`firewall_policies(Option<Vec<FirewallPolicyMetadata>>)`](crate::output::ListFirewallPoliciesOutput::firewall_policies): <p>The metadata for the firewall policies. Depending on your setting for max results and the number of firewall policies that you have, this might not be the full list. </p>
                            /// - On failure, responds with [`SdkError<ListFirewallPoliciesError>`](crate::error::ListFirewallPoliciesError)
    pub fn list_firewall_policies(&self) -> crate::client::fluent_builders::ListFirewallPolicies {
                                crate::client::fluent_builders::ListFirewallPolicies::new(self.handle.clone())
                            }
}

