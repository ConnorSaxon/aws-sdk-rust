// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRoute`](crate::client::fluent_builders::DeleteRoute) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_cidr_block(impl Into<String>)`](crate::client::fluent_builders::DeleteRoute::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::client::fluent_builders::DeleteRoute::set_destination_cidr_block): <p>The IPv4 CIDR range for the route. The value you specify must match the CIDR for the route exactly.</p>
    ///   - [`destination_ipv6_cidr_block(impl Into<String>)`](crate::client::fluent_builders::DeleteRoute::destination_ipv6_cidr_block) / [`set_destination_ipv6_cidr_block(Option<String>)`](crate::client::fluent_builders::DeleteRoute::set_destination_ipv6_cidr_block): <p>The IPv6 CIDR range for the route. The value you specify must match the CIDR for the route exactly.</p>
    ///   - [`destination_prefix_list_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRoute::destination_prefix_list_id) / [`set_destination_prefix_list_id(Option<String>)`](crate::client::fluent_builders::DeleteRoute::set_destination_prefix_list_id): <p>The ID of the prefix list for the route.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeleteRoute::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeleteRoute::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`route_table_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRoute::route_table_id) / [`set_route_table_id(Option<String>)`](crate::client::fluent_builders::DeleteRoute::set_route_table_id): <p>The ID of the route table.</p>
                            /// - On success, responds with [`DeleteRouteOutput`](crate::output::DeleteRouteOutput)
                            /// - On failure, responds with [`SdkError<DeleteRouteError>`](crate::error::DeleteRouteError)
    pub fn delete_route(&self) -> crate::client::fluent_builders::DeleteRoute {
                                crate::client::fluent_builders::DeleteRoute::new(self.handle.clone())
                            }
}

