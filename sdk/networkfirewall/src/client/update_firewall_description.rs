// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFirewallDescription`](crate::client::fluent_builders::UpdateFirewallDescription) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`update_token(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::update_token) / [`set_update_token(Option<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::set_update_token): <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>  <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>  <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p>
    ///   - [`firewall_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::firewall_arn) / [`set_firewall_arn(Option<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::set_firewall_arn): <p>The Amazon Resource Name (ARN) of the firewall.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_name(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::firewall_name) / [`set_firewall_name(Option<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::set_firewall_name): <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateFirewallDescription::set_description): <p>The new description for the firewall. If you omit this setting, Network Firewall removes the description for the firewall.</p>
                            /// - On success, responds with [`UpdateFirewallDescriptionOutput`](crate::output::UpdateFirewallDescriptionOutput) with field(s):
    ///   - [`firewall_arn(Option<String>)`](crate::output::UpdateFirewallDescriptionOutput::firewall_arn): <p>The Amazon Resource Name (ARN) of the firewall.</p>
    ///   - [`firewall_name(Option<String>)`](crate::output::UpdateFirewallDescriptionOutput::firewall_name): <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
    ///   - [`description(Option<String>)`](crate::output::UpdateFirewallDescriptionOutput::description): <p>A description of the firewall.</p>
    ///   - [`update_token(Option<String>)`](crate::output::UpdateFirewallDescriptionOutput::update_token): <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>  <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>  <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p>
                            /// - On failure, responds with [`SdkError<UpdateFirewallDescriptionError>`](crate::error::UpdateFirewallDescriptionError)
    pub fn update_firewall_description(&self) -> crate::client::fluent_builders::UpdateFirewallDescription {
                                crate::client::fluent_builders::UpdateFirewallDescription::new(self.handle.clone())
                            }
}

