// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFirewallPolicy`](crate::client::fluent_builders::DeleteFirewallPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`firewall_policy_name(impl Into<String>)`](crate::client::fluent_builders::DeleteFirewallPolicy::firewall_policy_name) / [`set_firewall_policy_name(Option<String>)`](crate::client::fluent_builders::DeleteFirewallPolicy::set_firewall_policy_name): <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_policy_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteFirewallPolicy::firewall_policy_arn) / [`set_firewall_policy_arn(Option<String>)`](crate::client::fluent_builders::DeleteFirewallPolicy::set_firewall_policy_arn): <p>The Amazon Resource Name (ARN) of the firewall policy.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
                            /// - On success, responds with [`DeleteFirewallPolicyOutput`](crate::output::DeleteFirewallPolicyOutput) with field(s):
    ///   - [`firewall_policy_response(Option<FirewallPolicyResponse>)`](crate::output::DeleteFirewallPolicyOutput::firewall_policy_response): <p>The object containing the definition of the <code>FirewallPolicyResponse</code> that you asked to delete. </p>
                            /// - On failure, responds with [`SdkError<DeleteFirewallPolicyError>`](crate::error::DeleteFirewallPolicyError)
    pub fn delete_firewall_policy(&self) -> crate::client::fluent_builders::DeleteFirewallPolicy {
                                crate::client::fluent_builders::DeleteFirewallPolicy::new(self.handle.clone())
                            }
}

