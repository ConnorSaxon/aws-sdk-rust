// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetNetworkResourceRelationships`](crate::client::fluent_builders::GetNetworkResourceRelationships) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetNetworkResourceRelationships::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`core_network_id(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::core_network_id) / [`set_core_network_id(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_core_network_id): <p>The ID of a core network.</p>
    ///   - [`registered_gateway_arn(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::registered_gateway_arn) / [`set_registered_gateway_arn(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_registered_gateway_arn): <p>The ARN of the registered gateway.</p>
    ///   - [`aws_region(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::aws_region) / [`set_aws_region(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_aws_region): <p>The Amazon Web Services Region.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_account_id): <p>The Amazon Web Services account ID.</p>
    ///   - [`resource_type(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::resource_type) / [`set_resource_type(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_resource_type): <p>The resource type.</p>  <p>The following are the supported resource types for Direct Connect:</p>  <ul>   <li> <p> <code>dxcon</code> </p> </li>   <li> <p> <code>dx-gateway</code> </p> </li>   <li> <p> <code>dx-vif</code> </p> </li>  </ul>  <p>The following are the supported resource types for Network Manager:</p>  <ul>   <li> <p> <code>connection</code> </p> </li>   <li> <p> <code>device</code> </p> </li>   <li> <p> <code>link</code> </p> </li>   <li> <p> <code>site</code> </p> </li>  </ul>  <p>The following are the supported resource types for Amazon VPC:</p>  <ul>   <li> <p> <code>customer-gateway</code> </p> </li>   <li> <p> <code>transit-gateway</code> </p> </li>   <li> <p> <code>transit-gateway-attachment</code> </p> </li>   <li> <p> <code>transit-gateway-connect-peer</code> </p> </li>   <li> <p> <code>transit-gateway-route-table</code> </p> </li>   <li> <p> <code>vpn-connection</code> </p> </li>  </ul>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_resource_arn): <p>The ARN of the gateway.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetNetworkResourceRelationships::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetNetworkResourceRelationships::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`GetNetworkResourceRelationshipsOutput`](crate::output::GetNetworkResourceRelationshipsOutput) with field(s):
    ///   - [`relationships(Option<Vec<Relationship>>)`](crate::output::GetNetworkResourceRelationshipsOutput::relationships): <p>The resource relationships.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetNetworkResourceRelationshipsOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<GetNetworkResourceRelationshipsError>`](crate::error::GetNetworkResourceRelationshipsError)
    pub fn get_network_resource_relationships(&self) -> crate::client::fluent_builders::GetNetworkResourceRelationships {
                                crate::client::fluent_builders::GetNetworkResourceRelationships::new(self.handle.clone())
                            }
}

