// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AuthorizeSecurityGroupEgress`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`group_id(impl Into<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::group_id) / [`set_group_id(Option<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_group_id): <p>The ID of the security group.</p>
    ///   - [`ip_permissions(Vec<IpPermission>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::ip_permissions) / [`set_ip_permissions(Option<Vec<IpPermission>>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_ip_permissions): <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_tag_specifications): <p>The tags applied to the security group rule.</p>
    ///   - [`cidr_ip(impl Into<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::cidr_ip) / [`set_cidr_ip(Option<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_cidr_ip): <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    ///   - [`from_port(i32)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::from_port) / [`set_from_port(Option<i32>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_from_port): <p>Not supported. Use a set of IP permissions to specify the port.</p>
    ///   - [`ip_protocol(impl Into<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::ip_protocol) / [`set_ip_protocol(Option<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_ip_protocol): <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    ///   - [`to_port(i32)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::to_port) / [`set_to_port(Option<i32>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_to_port): <p>Not supported. Use a set of IP permissions to specify the port.</p>
    ///   - [`source_security_group_name(impl Into<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::source_security_group_name) / [`set_source_security_group_name(Option<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_source_security_group_name): <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    ///   - [`source_security_group_owner_id(impl Into<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::source_security_group_owner_id) / [`set_source_security_group_owner_id(Option<String>)`](crate::client::fluent_builders::AuthorizeSecurityGroupEgress::set_source_security_group_owner_id): <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
                            /// - On success, responds with [`AuthorizeSecurityGroupEgressOutput`](crate::output::AuthorizeSecurityGroupEgressOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::output::AuthorizeSecurityGroupEgressOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    ///   - [`security_group_rules(Option<Vec<SecurityGroupRule>>)`](crate::output::AuthorizeSecurityGroupEgressOutput::security_group_rules): <p>Information about the outbound (egress) security group rules that were added.</p>
                            /// - On failure, responds with [`SdkError<AuthorizeSecurityGroupEgressError>`](crate::error::AuthorizeSecurityGroupEgressError)
    pub fn authorize_security_group_egress(&self) -> crate::client::fluent_builders::AuthorizeSecurityGroupEgress {
                                crate::client::fluent_builders::AuthorizeSecurityGroupEgress::new(self.handle.clone())
                            }
}

