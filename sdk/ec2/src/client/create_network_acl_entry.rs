// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNetworkAclEntry`](crate::client::fluent_builders::CreateNetworkAclEntry) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cidr_block(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::cidr_block) / [`set_cidr_block(Option<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_cidr_block): <p>The IPv4 network range to allow or deny, in CIDR notation (for example <code>172.16.0.0/24</code>). We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateNetworkAclEntry::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`egress(bool)`](crate::client::fluent_builders::CreateNetworkAclEntry::egress) / [`set_egress(Option<bool>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_egress): <p>Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet).</p>
    ///   - [`icmp_type_code(IcmpTypeCode)`](crate::client::fluent_builders::CreateNetworkAclEntry::icmp_type_code) / [`set_icmp_type_code(Option<IcmpTypeCode>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_icmp_type_code): <p>ICMP protocol: The ICMP or ICMPv6 type and code. Required if specifying protocol 1 (ICMP) or protocol 58 (ICMPv6) with an IPv6 CIDR block.</p>
    ///   - [`ipv6_cidr_block(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::ipv6_cidr_block) / [`set_ipv6_cidr_block(Option<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_ipv6_cidr_block): <p>The IPv6 network range to allow or deny, in CIDR notation (for example <code>2001:db8:1234:1a00::/64</code>).</p>
    ///   - [`network_acl_id(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::network_acl_id) / [`set_network_acl_id(Option<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_network_acl_id): <p>The ID of the network ACL.</p>
    ///   - [`port_range(PortRange)`](crate::client::fluent_builders::CreateNetworkAclEntry::port_range) / [`set_port_range(Option<PortRange>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_port_range): <p>TCP or UDP protocols: The range of ports the rule applies to. Required if specifying protocol 6 (TCP) or 17 (UDP).</p>
    ///   - [`protocol(impl Into<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::protocol) / [`set_protocol(Option<String>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_protocol): <p>The protocol number. A value of "-1" means all protocols. If you specify "-1" or a protocol number other than "6" (TCP), "17" (UDP), or "1" (ICMP), traffic on all ports is allowed, regardless of any ports or ICMP types or codes that you specify. If you specify protocol "58" (ICMPv6) and specify an IPv4 CIDR block, traffic for all ICMP types and codes allowed, regardless of any that you specify. If you specify protocol "58" (ICMPv6) and specify an IPv6 CIDR block, you must specify an ICMP type and code.</p>
    ///   - [`rule_action(RuleAction)`](crate::client::fluent_builders::CreateNetworkAclEntry::rule_action) / [`set_rule_action(Option<RuleAction>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_rule_action): <p>Indicates whether to allow or deny the traffic that matches the rule.</p>
    ///   - [`rule_number(i32)`](crate::client::fluent_builders::CreateNetworkAclEntry::rule_number) / [`set_rule_number(Option<i32>)`](crate::client::fluent_builders::CreateNetworkAclEntry::set_rule_number): <p>The rule number for the entry (for example, 100). ACL entries are processed in ascending order by rule number.</p>  <p>Constraints: Positive integer from 1 to 32766. The range 32767 to 65535 is reserved for internal use.</p>
                            /// - On success, responds with [`CreateNetworkAclEntryOutput`](crate::output::CreateNetworkAclEntryOutput)
                            /// - On failure, responds with [`SdkError<CreateNetworkAclEntryError>`](crate::error::CreateNetworkAclEntryError)
    pub fn create_network_acl_entry(&self) -> crate::client::fluent_builders::CreateNetworkAclEntry {
                                crate::client::fluent_builders::CreateNetworkAclEntry::new(self.handle.clone())
                            }
}

