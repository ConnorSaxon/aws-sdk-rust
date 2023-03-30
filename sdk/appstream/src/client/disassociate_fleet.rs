// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateFleet`](crate::client::fluent_builders::DisassociateFleet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_name(impl Into<String>)`](crate::client::fluent_builders::DisassociateFleet::fleet_name) / [`set_fleet_name(Option<String>)`](crate::client::fluent_builders::DisassociateFleet::set_fleet_name): <p>The name of the fleet.</p>
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::DisassociateFleet::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::DisassociateFleet::set_stack_name): <p>The name of the stack.</p>
                            /// - On success, responds with [`DisassociateFleetOutput`](crate::output::DisassociateFleetOutput)
                            /// - On failure, responds with [`SdkError<DisassociateFleetError>`](crate::error::DisassociateFleetError)
    pub fn disassociate_fleet(&self) -> crate::client::fluent_builders::DisassociateFleet {
                                crate::client::fluent_builders::DisassociateFleet::new(self.handle.clone())
                            }
}

