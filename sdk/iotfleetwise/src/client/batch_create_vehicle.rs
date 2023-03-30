// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchCreateVehicle`](crate::client::fluent_builders::BatchCreateVehicle) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`vehicles(Vec<CreateVehicleRequestItem>)`](crate::client::fluent_builders::BatchCreateVehicle::vehicles) / [`set_vehicles(Option<Vec<CreateVehicleRequestItem>>)`](crate::client::fluent_builders::BatchCreateVehicle::set_vehicles): <p> A list of information about each vehicle to create. For more information, see the API data type.</p>
                            /// - On success, responds with [`BatchCreateVehicleOutput`](crate::output::BatchCreateVehicleOutput) with field(s):
    ///   - [`vehicles(Option<Vec<CreateVehicleResponseItem>>)`](crate::output::BatchCreateVehicleOutput::vehicles): <p> A list of information about a batch of created vehicles. For more information, see the API data type.</p>
    ///   - [`errors(Option<Vec<CreateVehicleError>>)`](crate::output::BatchCreateVehicleOutput::errors): <p>A list of information about creation errors, or an empty list if there aren't any errors. </p>
                            /// - On failure, responds with [`SdkError<BatchCreateVehicleError>`](crate::error::BatchCreateVehicleError)
    pub fn batch_create_vehicle(&self) -> crate::client::fluent_builders::BatchCreateVehicle {
                                crate::client::fluent_builders::BatchCreateVehicle::new(self.handle.clone())
                            }
}

