// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFirewallConfigs`](crate::client::fluent_builders::ListFirewallConfigs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFirewallConfigs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFirewallConfigs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFirewallConfigs::set_max_results): <p>The maximum number of objects that you want Resolver to return for this request. If more objects are available, in the response, Resolver provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p>  <p>If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 objects. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFirewallConfigs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFirewallConfigs::set_next_token): <p>For the first call to this list request, omit this value.</p>  <p>When you request a list of objects, Resolver returns at most the number of objects specified in <code>MaxResults</code>. If more objects are available for retrieval, Resolver returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token that was returned for the prior request in your next request.</p>
                            /// - On success, responds with [`ListFirewallConfigsOutput`](crate::output::ListFirewallConfigsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListFirewallConfigsOutput::next_token): <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    ///   - [`firewall_configs(Option<Vec<FirewallConfig>>)`](crate::output::ListFirewallConfigsOutput::firewall_configs): <p>The configurations for the firewall behavior provided by DNS Firewall for VPCs from Amazon Virtual Private Cloud (Amazon VPC). </p>
                            /// - On failure, responds with [`SdkError<ListFirewallConfigsError>`](crate::error::ListFirewallConfigsError)
    pub fn list_firewall_configs(&self) -> crate::client::fluent_builders::ListFirewallConfigs {
                                crate::client::fluent_builders::ListFirewallConfigs::new(self.handle.clone())
                            }
}

