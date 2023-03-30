// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeFirewall`](crate::client::fluent_builders::DescribeFirewall) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`firewall_name(impl Into<String>)`](crate::client::fluent_builders::DescribeFirewall::firewall_name) / [`set_firewall_name(Option<String>)`](crate::client::fluent_builders::DescribeFirewall::set_firewall_name): <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeFirewall::firewall_arn) / [`set_firewall_arn(Option<String>)`](crate::client::fluent_builders::DescribeFirewall::set_firewall_arn): <p>The Amazon Resource Name (ARN) of the firewall.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
                            /// - On success, responds with [`DescribeFirewallOutput`](crate::output::DescribeFirewallOutput) with field(s):
    ///   - [`update_token(Option<String>)`](crate::output::DescribeFirewallOutput::update_token): <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>  <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>  <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p>
    ///   - [`firewall(Option<Firewall>)`](crate::output::DescribeFirewallOutput::firewall): <p>The configuration settings for the firewall. These settings include the firewall policy and the subnets in your VPC to use for the firewall endpoints. </p>
    ///   - [`firewall_status(Option<FirewallStatus>)`](crate::output::DescribeFirewallOutput::firewall_status): <p>Detailed information about the current status of a <code>Firewall</code>. You can retrieve this for a firewall by calling <code>DescribeFirewall</code> and providing the firewall name and ARN.</p>
                            /// - On failure, responds with [`SdkError<DescribeFirewallError>`](crate::error::DescribeFirewallError)
    pub fn describe_firewall(&self) -> crate::client::fluent_builders::DescribeFirewall {
                                crate::client::fluent_builders::DescribeFirewall::new(self.handle.clone())
                            }
}

