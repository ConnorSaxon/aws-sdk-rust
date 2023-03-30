// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateDhcpOptions`](crate::client::fluent_builders::AssociateDhcpOptions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dhcp_options_id(impl Into<String>)`](crate::client::fluent_builders::AssociateDhcpOptions::dhcp_options_id) / [`set_dhcp_options_id(Option<String>)`](crate::client::fluent_builders::AssociateDhcpOptions::set_dhcp_options_id): <p>The ID of the DHCP options set, or <code>default</code> to associate no DHCP options with the VPC.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::AssociateDhcpOptions::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::AssociateDhcpOptions::set_vpc_id): <p>The ID of the VPC.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AssociateDhcpOptions::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AssociateDhcpOptions::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`AssociateDhcpOptionsOutput`](crate::output::AssociateDhcpOptionsOutput)
                            /// - On failure, responds with [`SdkError<AssociateDhcpOptionsError>`](crate::error::AssociateDhcpOptionsError)
    pub fn associate_dhcp_options(&self) -> crate::client::fluent_builders::AssociateDhcpOptions {
                                crate::client::fluent_builders::AssociateDhcpOptions::new(self.handle.clone())
                            }
}

