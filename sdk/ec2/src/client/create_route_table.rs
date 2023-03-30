// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRouteTable`](crate::client::fluent_builders::CreateRouteTable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateRouteTable::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateRouteTable::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::CreateRouteTable::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::CreateRouteTable::set_vpc_id): <p>The ID of the VPC.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateRouteTable::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateRouteTable::set_tag_specifications): <p>The tags to assign to the route table.</p>
                            /// - On success, responds with [`CreateRouteTableOutput`](crate::output::CreateRouteTableOutput) with field(s):
    ///   - [`route_table(Option<RouteTable>)`](crate::output::CreateRouteTableOutput::route_table): <p>Information about the route table.</p>
                            /// - On failure, responds with [`SdkError<CreateRouteTableError>`](crate::error::CreateRouteTableError)
    pub fn create_route_table(&self) -> crate::client::fluent_builders::CreateRouteTable {
                                crate::client::fluent_builders::CreateRouteTable::new(self.handle.clone())
                            }
}

