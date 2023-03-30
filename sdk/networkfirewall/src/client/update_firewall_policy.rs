// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFirewallPolicy`](crate::client::fluent_builders::UpdateFirewallPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`update_token(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::update_token) / [`set_update_token(Option<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_update_token): <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the firewall policy. The token marks the state of the policy resource at the time of the request. </p>  <p>To make changes to the policy, you provide the token in your request. Network Firewall uses the token to ensure that the policy hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall policy again to get a current copy of it with current token. Reapply your changes as needed, then try the operation again using the new token. </p>
    ///   - [`firewall_policy_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::firewall_policy_arn) / [`set_firewall_policy_arn(Option<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_firewall_policy_arn): <p>The Amazon Resource Name (ARN) of the firewall policy.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_policy_name(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::firewall_policy_name) / [`set_firewall_policy_name(Option<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_firewall_policy_name): <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_policy(FirewallPolicy)`](crate::client::fluent_builders::UpdateFirewallPolicy::firewall_policy) / [`set_firewall_policy(Option<FirewallPolicy>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_firewall_policy): <p>The updated firewall policy to use for the firewall. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_description): <p>A description of the firewall policy.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::UpdateFirewallPolicy::dry_run) / [`set_dry_run(bool)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_dry_run): <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request. </p>  <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid. </p>  <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources. </p>
    ///   - [`encryption_configuration(EncryptionConfiguration)`](crate::client::fluent_builders::UpdateFirewallPolicy::encryption_configuration) / [`set_encryption_configuration(Option<EncryptionConfiguration>)`](crate::client::fluent_builders::UpdateFirewallPolicy::set_encryption_configuration): <p>A complex type that contains settings for encryption of your firewall policy resources.</p>
                            /// - On success, responds with [`UpdateFirewallPolicyOutput`](crate::output::UpdateFirewallPolicyOutput) with field(s):
    ///   - [`update_token(Option<String>)`](crate::output::UpdateFirewallPolicyOutput::update_token): <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the firewall policy. The token marks the state of the policy resource at the time of the request. </p>  <p>To make changes to the policy, you provide the token in your request. Network Firewall uses the token to ensure that the policy hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall policy again to get a current copy of it with current token. Reapply your changes as needed, then try the operation again using the new token. </p>
    ///   - [`firewall_policy_response(Option<FirewallPolicyResponse>)`](crate::output::UpdateFirewallPolicyOutput::firewall_policy_response): <p>The high-level properties of a firewall policy. This, along with the <code>FirewallPolicy</code>, define the policy. You can retrieve all objects for a firewall policy by calling <code>DescribeFirewallPolicy</code>. </p>
                            /// - On failure, responds with [`SdkError<UpdateFirewallPolicyError>`](crate::error::UpdateFirewallPolicyError)
    pub fn update_firewall_policy(&self) -> crate::client::fluent_builders::UpdateFirewallPolicy {
                                crate::client::fluent_builders::UpdateFirewallPolicy::new(self.handle.clone())
                            }
}

