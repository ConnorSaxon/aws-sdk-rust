// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UnassignPrivateIpAddresses`](crate::client::fluent_builders::UnassignPrivateIpAddresses) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_interface_id(impl Into<String>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::set_network_interface_id): <p>The ID of the network interface.</p>
    ///   - [`private_ip_addresses(Vec<String>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::private_ip_addresses) / [`set_private_ip_addresses(Option<Vec<String>>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::set_private_ip_addresses): <p>The secondary private IP addresses to unassign from the network interface. You can specify this option multiple times to unassign more than one IP address.</p>
    ///   - [`ipv4_prefixes(Vec<String>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::ipv4_prefixes) / [`set_ipv4_prefixes(Option<Vec<String>>)`](crate::client::fluent_builders::UnassignPrivateIpAddresses::set_ipv4_prefixes): <p>The IPv4 prefixes to unassign from the network interface.</p>
                            /// - On success, responds with [`UnassignPrivateIpAddressesOutput`](crate::output::UnassignPrivateIpAddressesOutput)
                            /// - On failure, responds with [`SdkError<UnassignPrivateIpAddressesError>`](crate::error::UnassignPrivateIpAddressesError)
    pub fn unassign_private_ip_addresses(&self) -> crate::client::fluent_builders::UnassignPrivateIpAddresses {
                                crate::client::fluent_builders::UnassignPrivateIpAddresses::new(self.handle.clone())
                            }
}

