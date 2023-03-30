// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateSubnets`](crate::client::fluent_builders::AssociateSubnets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`update_token(impl Into<String>)`](crate::client::fluent_builders::AssociateSubnets::update_token) / [`set_update_token(Option<String>)`](crate::client::fluent_builders::AssociateSubnets::set_update_token): <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>  <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>  <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p>
    ///   - [`firewall_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateSubnets::firewall_arn) / [`set_firewall_arn(Option<String>)`](crate::client::fluent_builders::AssociateSubnets::set_firewall_arn): <p>The Amazon Resource Name (ARN) of the firewall.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_name(impl Into<String>)`](crate::client::fluent_builders::AssociateSubnets::firewall_name) / [`set_firewall_name(Option<String>)`](crate::client::fluent_builders::AssociateSubnets::set_firewall_name): <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`subnet_mappings(Vec<SubnetMapping>)`](crate::client::fluent_builders::AssociateSubnets::subnet_mappings) / [`set_subnet_mappings(Option<Vec<SubnetMapping>>)`](crate::client::fluent_builders::AssociateSubnets::set_subnet_mappings): <p>The IDs of the subnets that you want to associate with the firewall. </p>
                            /// - On success, responds with [`AssociateSubnetsOutput`](crate::output::AssociateSubnetsOutput) with field(s):
    ///   - [`firewall_arn(Option<String>)`](crate::output::AssociateSubnetsOutput::firewall_arn): <p>The Amazon Resource Name (ARN) of the firewall.</p>
    ///   - [`firewall_name(Option<String>)`](crate::output::AssociateSubnetsOutput::firewall_name): <p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>
    ///   - [`subnet_mappings(Option<Vec<SubnetMapping>>)`](crate::output::AssociateSubnetsOutput::subnet_mappings): <p>The IDs of the subnets that are associated with the firewall. </p>
    ///   - [`update_token(Option<String>)`](crate::output::AssociateSubnetsOutput::update_token): <p>An optional token that you can use for optimistic locking. Network Firewall returns a token to your requests that access the firewall. The token marks the state of the firewall resource at the time of the request. </p>  <p>To make an unconditional change to the firewall, omit the token in your update request. Without the token, Network Firewall performs your updates regardless of whether the firewall has changed since you last retrieved it.</p>  <p>To make a conditional change to the firewall, provide the token in your update request. Network Firewall uses the token to ensure that the firewall hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the firewall again to get a current copy of it with a new token. Reapply your changes as needed, then try the operation again using the new token. </p>
                            /// - On failure, responds with [`SdkError<AssociateSubnetsError>`](crate::error::AssociateSubnetsError)
    pub fn associate_subnets(&self) -> crate::client::fluent_builders::AssociateSubnets {
                                crate::client::fluent_builders::AssociateSubnets::new(self.handle.clone())
                            }
}

