// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteClientVpnEndpoint`](crate::client::fluent_builders::DeleteClientVpnEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_vpn_endpoint_id(impl Into<String>)`](crate::client::fluent_builders::DeleteClientVpnEndpoint::client_vpn_endpoint_id) / [`set_client_vpn_endpoint_id(Option<String>)`](crate::client::fluent_builders::DeleteClientVpnEndpoint::set_client_vpn_endpoint_id): <p>The ID of the Client VPN to be deleted.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeleteClientVpnEndpoint::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeleteClientVpnEndpoint::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DeleteClientVpnEndpointOutput`](crate::output::DeleteClientVpnEndpointOutput) with field(s):
    ///   - [`status(Option<ClientVpnEndpointStatus>)`](crate::output::DeleteClientVpnEndpointOutput::status): <p>The current state of the Client VPN endpoint.</p>
                            /// - On failure, responds with [`SdkError<DeleteClientVpnEndpointError>`](crate::error::DeleteClientVpnEndpointError)
    pub fn delete_client_vpn_endpoint(&self) -> crate::client::fluent_builders::DeleteClientVpnEndpoint {
                                crate::client::fluent_builders::DeleteClientVpnEndpoint::new(self.handle.clone())
                            }
}

