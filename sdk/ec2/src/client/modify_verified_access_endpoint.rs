// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessEndpoint`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`verified_access_endpoint_id(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::verified_access_endpoint_id) / [`set_verified_access_endpoint_id(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_verified_access_endpoint_id): <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    ///   - [`verified_access_group_id(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::verified_access_group_id) / [`set_verified_access_group_id(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_verified_access_group_id): <p>The ID of the Amazon Web Services Verified Access group.</p>
    ///   - [`load_balancer_options(ModifyVerifiedAccessEndpointLoadBalancerOptions)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::load_balancer_options) / [`set_load_balancer_options(Option<ModifyVerifiedAccessEndpointLoadBalancerOptions>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_load_balancer_options): <p>The load balancer details if creating the Amazon Web Services Verified Access endpoint as <code>load-balancer</code>type.</p>
    ///   - [`network_interface_options(ModifyVerifiedAccessEndpointEniOptions)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::network_interface_options) / [`set_network_interface_options(Option<ModifyVerifiedAccessEndpointEniOptions>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_network_interface_options): <p>The network interface options.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_description): <p>A description for the Amazon Web Services Verified Access endpoint.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`ModifyVerifiedAccessEndpointOutput`](crate::output::ModifyVerifiedAccessEndpointOutput) with field(s):
    ///   - [`verified_access_endpoint(Option<VerifiedAccessEndpoint>)`](crate::output::ModifyVerifiedAccessEndpointOutput::verified_access_endpoint): <p>The Amazon Web Services Verified Access endpoint details.</p>
                            /// - On failure, responds with [`SdkError<ModifyVerifiedAccessEndpointError>`](crate::error::ModifyVerifiedAccessEndpointError)
    pub fn modify_verified_access_endpoint(&self) -> crate::client::fluent_builders::ModifyVerifiedAccessEndpoint {
                                crate::client::fluent_builders::ModifyVerifiedAccessEndpoint::new(self.handle.clone())
                            }
}

