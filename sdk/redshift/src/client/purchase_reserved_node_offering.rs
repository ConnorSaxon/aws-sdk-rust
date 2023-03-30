// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PurchaseReservedNodeOffering`](crate::client::fluent_builders::PurchaseReservedNodeOffering) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`reserved_node_offering_id(impl Into<String>)`](crate::client::fluent_builders::PurchaseReservedNodeOffering::reserved_node_offering_id) / [`set_reserved_node_offering_id(Option<String>)`](crate::client::fluent_builders::PurchaseReservedNodeOffering::set_reserved_node_offering_id): <p>The unique identifier of the reserved node offering you want to purchase.</p>
    ///   - [`node_count(i32)`](crate::client::fluent_builders::PurchaseReservedNodeOffering::node_count) / [`set_node_count(Option<i32>)`](crate::client::fluent_builders::PurchaseReservedNodeOffering::set_node_count): <p>The number of reserved nodes that you want to purchase.</p>  <p>Default: <code>1</code> </p>
                            /// - On success, responds with [`PurchaseReservedNodeOfferingOutput`](crate::output::PurchaseReservedNodeOfferingOutput) with field(s):
    ///   - [`reserved_node(Option<ReservedNode>)`](crate::output::PurchaseReservedNodeOfferingOutput::reserved_node): <p>Describes a reserved node. You can call the <code>DescribeReservedNodeOfferings</code> API to obtain the available reserved node offerings. </p>
                            /// - On failure, responds with [`SdkError<PurchaseReservedNodeOfferingError>`](crate::error::PurchaseReservedNodeOfferingError)
    pub fn purchase_reserved_node_offering(&self) -> crate::client::fluent_builders::PurchaseReservedNodeOffering {
                                crate::client::fluent_builders::PurchaseReservedNodeOffering::new(self.handle.clone())
                            }
}

