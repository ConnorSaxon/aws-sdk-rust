// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFleet`](crate::client::fluent_builders::UpdateFleet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::client::fluent_builders::UpdateFleet::fleet_id) / [`set_fleet_id(Option<String>)`](crate::client::fluent_builders::UpdateFleet::set_fleet_id): <p> The ID of the fleet to update. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateFleet::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateFleet::set_description): <p> An updated description of the fleet. </p>
                            /// - On success, responds with [`UpdateFleetOutput`](crate::output::UpdateFleetOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::UpdateFleetOutput::id): <p>The ID of the updated fleet.</p>
    ///   - [`arn(Option<String>)`](crate::output::UpdateFleetOutput::arn): <p>The Amazon Resource Name (ARN) of the updated fleet.</p>
                            /// - On failure, responds with [`SdkError<UpdateFleetError>`](crate::error::UpdateFleetError)
    pub fn update_fleet(&self) -> crate::client::fluent_builders::UpdateFleet {
                                crate::client::fluent_builders::UpdateFleet::new(self.handle.clone())
                            }
}

