// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFleet`](crate::client::fluent_builders::DeleteFleet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet(impl Into<String>)`](crate::client::fluent_builders::DeleteFleet::fleet) / [`set_fleet(Option<String>)`](crate::client::fluent_builders::DeleteFleet::set_fleet): <p>The Amazon Resource Name (ARN) of the fleet.</p>
                            /// - On success, responds with [`DeleteFleetOutput`](crate::output::DeleteFleetOutput)
                            /// - On failure, responds with [`SdkError<DeleteFleetError>`](crate::error::DeleteFleetError)
    #[deprecated(note = "Support for the AWS RoboMaker application deployment feature has ended. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/fleets.html.")]
    pub fn delete_fleet(&self) -> crate::client::fluent_builders::DeleteFleet {
                                crate::client::fluent_builders::DeleteFleet::new(self.handle.clone())
                            }
}

