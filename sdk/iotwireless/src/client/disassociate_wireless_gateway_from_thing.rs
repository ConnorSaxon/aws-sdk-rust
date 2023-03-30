// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateWirelessGatewayFromThing`](crate::client::fluent_builders::DisassociateWirelessGatewayFromThing) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DisassociateWirelessGatewayFromThing::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DisassociateWirelessGatewayFromThing::set_id): <p>The ID of the resource to update.</p>
                            /// - On success, responds with [`DisassociateWirelessGatewayFromThingOutput`](crate::output::DisassociateWirelessGatewayFromThingOutput)
                            /// - On failure, responds with [`SdkError<DisassociateWirelessGatewayFromThingError>`](crate::error::DisassociateWirelessGatewayFromThingError)
    pub fn disassociate_wireless_gateway_from_thing(&self) -> crate::client::fluent_builders::DisassociateWirelessGatewayFromThing {
                                crate::client::fluent_builders::DisassociateWirelessGatewayFromThing::new(self.handle.clone())
                            }
}

